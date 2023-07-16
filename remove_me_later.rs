
    let authorized_api = create_rw_signal(cx, None::<AuthorizedApi>);
    let user_info = create_rw_signal(cx, None::<FilteredUser>);
    let status = create_rw_signal(cx, None::<Status>);

    let loading = create_rw_signal(cx, LoadingState::Loading);

    provide_context(cx, authorized_api);
    provide_context(cx, status);
    provide_context(cx, loading);

    // -- actions -- //

    let fetch_user_info = create_action(cx, move |_| async move {
        loading.set(LoadingState::Loading);

        match authorized_api.get_untracked() {
            Some(api) => match api.user_info().await {
                Ok(info) => {
                    provide_context(cx, api.clone());
                    user_info.update(|i| *i = Some(info));
                }
                Err(err) => {
                    tracing::error!("Unable to fetch user info: {err}");
                }
            },
            None => {
                tracing::error!("Unable to fetch user info: not logged in");
            }
        }

        loading.set(LoadingState::Ready);
    });

    let logout = create_action(cx, move |_| async move {
        loading.set(LoadingState::Loading);

        // this is also outside a reactive tracking context for some reason
        match authorized_api.get() {
            Some(api) => match api.logout().await {
                Ok(_) => {
                    authorized_api.update(|a| *a = None);
                    user_info.update(|i| *i = None);
                }
                Err(err) => {
                    tracing::error!("Unable to logout: {err}");
                }
            },
            None => {
                tracing::error!("Unable to logout user: not logged in");
            }
        };

        loading.set(LoadingState::Ready);
    });

    let fetch_status = create_action(cx, move |_| async move {
        loading.set(LoadingState::Loading);

        match authorized_api.get_untracked() {
            Some(api) => match api.last_status().await {
                Ok(s) => {
                    status.set(Some(s));
                }
                Err(err) => {
                    tracing::error!("Unable to fetch status: {err}");
                }
            },
            None => {}
        };

        // This is probably the longest thing we have to load
        loading.set(LoadingState::Ready);
    });

    // Channel, Log lines
    let base_logs = create_rw_signal(cx, LogData::default());
    provide_context(cx, base_logs);

    let fetch_logs = create_action(cx, move |_| async move {
        loading.set(LoadingState::Loading);

        match authorized_api.get_untracked() {
            Some(api) => {
                let api = api.clone();
                if let Ok(new_data) = api.get_log("arma").await {
                    base_logs.update(|l| {
                        if !l.contains_key("arma") {
                            l.insert("arma".into(), vec![]);
                        }
                        l.get_mut("arma").unwrap().extend(new_data.log.clone())
                    });
                }

                if let Ok(new_data) = api.get_log("steamcmd").await {
                    base_logs.update(|l| {
                        if !l.contains_key("steamcmd") {
                            l.insert("steamcmd".into(), vec![]);
                        }
                        l.get_mut("steamcmd").unwrap().extend(new_data.log.clone())
                    });
                }

                let uri = format!("{}/logs?token={}", DEFAULT_SSE_URL, api.token().token);
                let mut event_source = EventSource::new(&uri).expect("EventSource::new");

                let mut steamcmd_stream = event_source.subscribe("steamcmd").unwrap();
                let mut arma_stream = event_source.subscribe("arma").unwrap();

                let base_logs = base_logs.clone();
                spawn_local(async move {
                    let _ = event_source.state();
                    let mut all_streams = stream::select(steamcmd_stream, arma_stream);
                    tracing::info!("Starting eventsource");
                    while let Some(Ok((event_type, msg))) = all_streams.next().await {
                        let new_data = msg.data().as_string().unwrap();
                        let new_data = serde_json::from_str::<Vec<String>>(&new_data).unwrap();
                        base_logs.update(|l| {
                            if !l.contains_key(&event_type) {
                                l.insert(event_type.clone().into(), vec![]);
                            }
                            l.get_mut(&event_type).unwrap().extend(new_data.clone())
                        });
                    }
                    tracing::info!("Ending eventsource");
                });
            }
            None => {}
        };

        // This is probably the longest thing we have to load
        loading.set(LoadingState::Ready);
    });

    // -- theme management -- //

    let (theme, set_theme) = create_signal(cx, Theme::Default);
    provide_context(cx, theme);
    provide_context(cx, set_theme);

    create_effect(cx, move |_| {
        let theme = match LocalStorage::get("theme") {
            Ok(theme) => theme,
            Err(e) => Theme::Dark,
        };
        set_theme.set(theme);
    });

    let html_attributes =
        create_rw_signal::<AdditionalAttributes>(cx, vec![("data-theme", move || theme.get().to_string())].into());

    // -- callbacks -- //

    let on_logout = move || {
        logout.dispatch(());
    };

    // -- init API -- //

    let unauthorized_api = UnauthorizedApi::new(DEFAULT_API_URL);

    create_effect(cx, move |_| {
        loading.set(LoadingState::Loading);
        if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY) {
            let api = AuthorizedApi::new(DEFAULT_API_URL, token);
            authorized_api.update(|a| *a = Some(api));
            fetch_status.dispatch(());
            fetch_user_info.dispatch(());
            fetch_logs.dispatch(());
        }
        loading.set(LoadingState::Ready);
    });

    // -- status -- //

    create_effect(cx, move |_| {
        let abort = oneshot::channel();
        match authorized_api.get() {
            Some(api) => {
                let url = format!("{}/status?token={}", DEFAULT_SSE_URL, api.token().token);

                let mut event_source = EventSource::new(&url).expect("EventSource::new");
                let mut stream = event_source.subscribe("message").unwrap();

                spawn_local(async move {
                    let _ = event_source.state(); // this blocks until connected?
                    // select abort or stream.next()
                    let mut stream = stream::select(stream, abort.1);
                    


                    while let Some(Ok((event_type, msg))) = stream.next().await {
                        if authorized_api.get_untracked().is_none() {
                            break;
                        }

                        status.set(Some(serde_json::from_str(&msg.data().as_string().unwrap()).unwrap()));
                    }
                });
            }
            None => {
                abort.0.send(());
            }
        }
    });

    // -- effects -- //

    create_effect(cx, move |_| {
        tracing::debug!("API authorization state changed");
        match authorized_api.get() {
            Some(api) => {
                tracing::debug!("API is now authorized: save token in LocalStorage");
                LocalStorage::set(API_TOKEN_STORAGE_KEY, api.token()).expect("LocalStorage::set");
            }
            None => {
                tracing::debug!(
                    "API is no longer authorized: delete token from \
                     LocalStorage"
                );
                LocalStorage::delete(API_TOKEN_STORAGE_KEY);
            }
        }
    });

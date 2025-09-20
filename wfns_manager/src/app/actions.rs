#[macro_export]
macro_rules! init_app_actions {
    ($sender:expr, $widgets:expr) => {
        let app = relm4::main_application();
        app.set_accelerators_for_action::<NextSFAction>(&["Right"]);
        app.set_accelerators_for_action::<PrevSFAction>(&["Left"]);
        app.set_accelerators_for_action::<OpenNewDir>(&["<ctrl><shift>o"]);
        app.set_accelerators_for_action::<OpenSFImg>(&["Up"]);
        app.set_accelerators_for_action::<OpenSF>(&["<ctrl>e"]);

        let next_sender = $sender.clone();
        let action_next: RelmAction<NextSFAction> =
            RelmAction::new_stateless(move |_| next_sender.input(AppInput::NextItem));

        let prev_sender = $sender.clone();
        let action_prev: RelmAction<PrevSFAction> =
            RelmAction::new_stateless(move |_| prev_sender.input(AppInput::PrevItem));

        let open_sender = $sender.clone();
        let action_open: RelmAction<OpenNewDir> =
            RelmAction::new_stateless(move |_| open_sender.input(AppInput::ChooseFolder));

        let open_sf_img_sender = $sender.clone();
        let action_open_sf_img: RelmAction<OpenSFImg> =
            RelmAction::new_stateless(move |_| open_sf_img_sender.input(AppInput::OpenImg));

        let open_sf_sender = $sender.clone();
        let action_open_sf: RelmAction<OpenSF> =
            RelmAction::new_stateless(move |_| open_sf_sender.input(AppInput::OpenDir));

        let mut alone_group = RelmActionGroup::<ShortcutsActionGroup>::new();
        alone_group.add_action(action_next);
        alone_group.add_action(action_prev);
        alone_group.add_action(action_open);
        alone_group.add_action(action_open_sf_img);
        alone_group.add_action(action_open_sf);
        alone_group.register_for_widget(&$widgets.main_window);
    };
}

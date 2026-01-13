use bevy::prelude::*;

/// Fileメニューの構築
pub fn build_file_menu(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        },
        Name::new("FileMenu"),
    )).with_children(|file_menu| {
        file_menu.spawn((
            Text::new("File"),
            bevy::text::TextFont {
                font_size: 14.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
        
        // メニュー項目（簡易実装：クリック可能なボタンとして表示）
        // New Project
        file_menu.spawn((
            Node {
                width: Val::Px(100.0),
                height: Val::Px(20.0),
                margin: UiRect::left(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            Interaction::default(),
            Name::new("MenuNewProject"),
        )).with_children(|button| {
            button.spawn((
                Text::new("New Project"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // Open Project
        file_menu.spawn((
            Node {
                width: Val::Px(100.0),
                height: Val::Px(20.0),
                margin: UiRect::left(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            Interaction::default(),
            Name::new("MenuOpenProject"),
        )).with_children(|button| {
            button.spawn((
                Text::new("Open Project"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}

/// Editメニューの構築
pub fn build_edit_menu(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        },
        Name::new("EditMenu"),
    )).with_children(|edit_menu| {
        edit_menu.spawn((
            Text::new("Edit"),
            bevy::text::TextFont {
                font_size: 14.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
        
        // 設定メニュー項目
        edit_menu.spawn((
            Node {
                width: Val::Px(100.0),
                height: Val::Px(20.0),
                margin: UiRect::left(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            Interaction::default(),
            Name::new("MenuSettings"),
        )).with_children(|button| {
            button.spawn((
                Text::new("Settings"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}

/// Buildメニューの構築
pub fn build_build_menu(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        },
        Name::new("BuildMenu"),
    )).with_children(|build_menu| {
        build_menu.spawn((
            Text::new("Build"),
            bevy::text::TextFont {
                font_size: 14.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
        
        // Build Game
        build_menu.spawn((
            Node {
                width: Val::Px(120.0),
                height: Val::Px(20.0),
                margin: UiRect::left(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            Interaction::default(),
            Name::new("MenuBuildGame"),
        )).with_children(|button| {
            button.spawn((
                Text::new("Build Game"),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}

/// Viewメニューの構築
pub fn build_view_menu(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        },
        Name::new("ViewMenu"),
    )).with_children(|view_menu| {
        view_menu.spawn((
            Text::new("View"),
            bevy::text::TextFont {
                font_size: 14.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
    });
}

/// Helpメニューの構築
pub fn build_help_menu(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        },
        Name::new("HelpMenu"),
    )).with_children(|help_menu| {
        help_menu.spawn((
            Text::new("Help"),
            bevy::text::TextFont {
                font_size: 14.0,
                ..default()
            },
            bevy::text::TextColor(Color::WHITE),
        ));
    });
}

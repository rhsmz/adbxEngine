use bevy::prelude::*;

/// ファイル操作グループのボタン配置
pub fn build_file_operation_buttons(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(5.0)),
            align_items: AlignItems::Center,
            border: UiRect::right(Val::Px(1.0)),
            margin: UiRect::right(Val::Px(5.0)),
            ..default()
        },
        bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
    )).with_children(|file_group| {
        // 新規ファイル
        file_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Interaction::default(),
            Name::new("ToolbarNewFile"),
        )).with_children(|button| {
            button.spawn((
                Text::new("📄"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // ファイルを開く
        file_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Interaction::default(),
            Name::new("ToolbarOpenFile"),
        )).with_children(|button| {
            button.spawn((
                Text::new("📂"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // 保存
        file_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Interaction::default(),
            Name::new("ToolbarSave"),
        )).with_children(|button| {
            button.spawn((
                Text::new("💾"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}

/// 編集操作グループのボタン配置
pub fn build_edit_operation_buttons(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(5.0)),
            align_items: AlignItems::Center,
            border: UiRect::right(Val::Px(1.0)),
            margin: UiRect::right(Val::Px(5.0)),
            ..default()
        },
        bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
    )).with_children(|edit_group| {
        // Undo
        edit_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Interaction::default(),
            Name::new("ToolbarUndo"),
        )).with_children(|button| {
            button.spawn((
                Text::new("↶"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // Redo
        edit_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Interaction::default(),
            Name::new("ToolbarRedo"),
        )).with_children(|button| {
            button.spawn((
                Text::new("↷"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}

/// 実行操作グループのボタン配置
pub fn build_run_operation_buttons(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(5.0)),
            align_items: AlignItems::Center,
            border: UiRect::right(Val::Px(1.0)),
            margin: UiRect::right(Val::Px(5.0)),
            ..default()
        },
        bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
    )).with_children(|run_group| {
        // 実行/再生
        run_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.5, 0.25)),
            Interaction::default(),
            Name::new("ToolbarPlay"),
        )).with_children(|button| {
            button.spawn((
                Text::new("▶"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
        
        // 停止
        run_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.25, 0.25)),
            Interaction::default(),
            Name::new("ToolbarStop"),
        )).with_children(|button| {
            button.spawn((
                Text::new("⏹"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}

/// ビュー操作グループのボタン配置
pub fn build_view_operation_buttons(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            padding: UiRect::horizontal(Val::Px(5.0)),
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|view_group| {
        // パネルの表示/非表示切り替え
        view_group.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            Interaction::default(),
            Name::new("ToolbarTogglePanels"),
        )).with_children(|button| {
            button.spawn((
                Text::new("⊞"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}

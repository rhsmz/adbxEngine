use super::super::AssetBrowser;
use bevy::prelude::*;
use bevy::ui::Interaction;

/// ヘッダー部分の描画（パス表示、戻るボタン、インポート/エクスポートボタン）
pub fn draw_asset_browser_header(parent: &mut ChildSpawnerCommands, asset_browser: &AssetBrowser) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(5.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            Name::new("AssetBrowserPath"),
        ))
        .with_children(|path_node: &mut ChildSpawnerCommands| {
            // 戻るボタン（親ディレクトリがある場合のみ表示）
            if asset_browser.current_path.parent().is_some() {
                path_node
                    .spawn((
                        Node {
                            width: Val::Px(60.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        Interaction::default(),
                        Name::new("AssetBrowserBackButton"),
                    ))
                    .with_children(|button: &mut ChildSpawnerCommands| {
                        button.spawn((
                            Text::new("← Back"),
                            bevy::text::TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            bevy::text::TextColor(Color::WHITE),
                        ));
                    });
            }

            // インポートボタン
            path_node
                .spawn((
                    Node {
                        width: Val::Px(70.0),
                        height: Val::Px(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                    Interaction::default(),
                    Name::new("AssetBrowserImportButton"),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Import"),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });

            // エクスポートボタン（選択されたアセットがある場合のみ表示）
            if asset_browser.selected_asset.is_some() {
                path_node
                    .spawn((
                        Node {
                            width: Val::Px(70.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.5)),
                        Interaction::default(),
                        Name::new("AssetBrowserExportButton"),
                    ))
                    .with_children(|button: &mut ChildSpawnerCommands| {
                        button.spawn((
                            Text::new("Export"),
                            bevy::text::TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            bevy::text::TextColor(Color::WHITE),
                        ));
                    });
            }

            // パス表示
            path_node
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Px(5.0)),
                        ..default()
                    },
                    Name::new("AssetBrowserPathText"),
                ))
                .with_children(|path_text: &mut ChildSpawnerCommands| {
                    path_text.spawn((
                        Text::new(asset_browser.current_path.to_string_lossy().as_ref()),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });
        });
}

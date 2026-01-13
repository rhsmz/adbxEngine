use bevy::prelude::*;

/// テクスチャプレビューの描画
pub fn draw_texture_preview(
    parent: &mut ChildSpawnerCommands,
    image_handle_opt: Option<bevy::asset::Handle<Image>>,
    image_loaded: bool,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            Name::new("AssetBrowserImagePreview"),
        ))
        .with_children(|image_preview: &mut ChildSpawnerCommands| {
            if let Some(image_handle) = image_handle_opt {
                // 画像が読み込まれているかチェック
                if image_loaded {
                    // 画像を表示（ImageNodeを使用）
                    image_preview
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            Name::new("AssetBrowserImageContainer"),
                        ))
                        .with_children(|container: &mut ChildSpawnerCommands| {
                            container.spawn((
                                ImageNode::new(image_handle),
                                Node {
                                    width: Val::Auto,
                                    height: Val::Auto,
                                    max_width: Val::Percent(100.0),
                                    max_height: Val::Percent(100.0),
                                    ..default()
                                },
                                Name::new("AssetBrowserImage"),
                            ));
                        });
                } else {
                    // 画像がまだ読み込まれていない場合、プレースホルダーを表示
                    image_preview
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Name::new("AssetBrowserImageContainer"),
                        ))
                        .with_children(|container: &mut ChildSpawnerCommands| {
                            container.spawn((
                                Text::new("🖼️\n画像を読み込み中..."),
                                bevy::text::TextFont {
                                    font_size: 11.0,
                                    ..default()
                                },
                                bevy::text::TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                Name::new("AssetBrowserImagePlaceholder"),
                            ));
                        });
                }
            } else {
                // パスが無効な場合
                image_preview.spawn((
                    Text::new("🖼️\n画像パスが無効です"),
                    bevy::text::TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::srgb(0.7, 0.3, 0.3)),
                    Name::new("AssetBrowserImageError"),
                ));
            }
        });
}

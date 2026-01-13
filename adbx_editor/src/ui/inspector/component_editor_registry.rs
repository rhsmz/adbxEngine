use bevy::prelude::*;
use std::collections::HashMap;
use std::any::TypeId;

/// コンポーネントエディタのトレイト
/// 各Componentタイプに対して、カスタムエディタを実装できます
pub trait ComponentEditor: Send + Sync {
    /// コンポーネントのUIを描画
    fn draw_ui(
        &self,
        commands: &mut Commands,
        entity: Entity,
        parent_ui: Entity,
    );
    
    /// コンポーネントの名前を取得
    fn component_name(&self) -> &'static str;
}

/// コンポーネントエディタの登録システム
#[derive(Resource, Default)]
pub struct ComponentEditorRegistry {
    editors: HashMap<TypeId, Box<dyn ComponentEditor>>,
}

impl ComponentEditorRegistry {
    /// 新しいエディタを登録
    pub fn register<T: Component + 'static>(&mut self, editor: Box<dyn ComponentEditor>) {
        self.editors.insert(TypeId::of::<T>(), editor);
    }
    
    /// エディタを取得
    pub fn get<T: Component + 'static>(&self) -> Option<&dyn ComponentEditor> {
        self.editors.get(&TypeId::of::<T>()).map(|e| e.as_ref())
    }
    
    /// すべてのエディタを取得
    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &dyn ComponentEditor)> {
        self.editors.iter().map(|(k, v)| (k, v.as_ref()))
    }
}

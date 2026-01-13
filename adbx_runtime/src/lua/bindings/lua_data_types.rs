use mlua::{UserData, UserDataMethods};

/// Transformデータ（Lua用）
#[derive(Clone)]
pub struct TransformData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl UserData for TransformData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("x", |_, this, ()| Ok(this.x));
        methods.add_method("y", |_, this, ()| Ok(this.y));
        methods.add_method("z", |_, this, ()| Ok(this.z));
        methods.add_method_mut("set_x", |_, this, val: f32| {
            this.x = val;
            Ok(())
        });
        methods.add_method_mut("set_y", |_, this, val: f32| {
            this.y = val;
            Ok(())
        });
        methods.add_method_mut("set_z", |_, this, val: f32| {
            this.z = val;
            Ok(())
        });
    }
}

/// Vec3データ（Lua用）
#[derive(Clone)]
pub struct Vec3Data {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl UserData for Vec3Data {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("x", |_, this, ()| Ok(this.x));
        methods.add_method("y", |_, this, ()| Ok(this.y));
        methods.add_method("z", |_, this, ()| Ok(this.z));
        methods.add_method("length", |_, this, ()| {
            Ok((this.x * this.x + this.y * this.y + this.z * this.z).sqrt())
        });
        methods.add_method("normalize", |_, this, ()| {
            let len = (this.x * this.x + this.y * this.y + this.z * this.z).sqrt();
            if len > 0.0 {
                Ok(Vec3Data {
                    x: this.x / len,
                    y: this.y / len,
                    z: this.z / len,
                })
            } else {
                Ok(Vec3Data {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                })
            }
        });
    }
}

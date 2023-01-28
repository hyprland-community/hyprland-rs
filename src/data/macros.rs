macro_rules! impl_on {
    ($name:ident) => {
        #[async_trait]
        impl HyprData for $name {
            fn get() -> HResult<Self> {
                let data = call_hyprctl_data_cmd(DataCommands::$name);
                let deserialized: $name = serde_json::from_str(&data)?;
                Ok(deserialized)
            }
            async fn get_async() -> HResult<Self> {
                let data = call_hyprctl_data_cmd_async(DataCommands::$name).await;
                let deserialized: $name = serde_json::from_str(&data)?;
                Ok(deserialized)
            }
        }
    };
}

macro_rules! create_data_struct {
    (vec $name:ident,$kind:path,$held:ty,$c:literal) => {
        #[doc = $c]
        #[derive(Debug, Clone)]
        pub struct $name {
            pos: usize,
            held: Vec<$held>,
        }

        #[async_trait]
        impl HyprData for $name {
            fn get() -> HResult<Self> {
                let data = call_hyprctl_data_cmd($kind);
                let deserialized: Vec<$held> = serde_json::from_str(&data)?;
                Ok(Self {
                    held: deserialized,
                    pos: 0,
                })
            }
            async fn get_async() -> HResult<Self> {
                let data = call_hyprctl_data_cmd_async($kind).await;
                let deserialized: Vec<$held> = serde_json::from_str(&data)?;
                Ok(Self {
                    held: deserialized,
                    pos: 0,
                })
            }
        }

        impl Iterator for $name {
            type Item = $held;

            fn next(&mut self) -> Option<Self::Item> {
                let out = self.held.get(self.pos);
                self.pos += 1;
                match out {
                    Some(v) => Some(v.clone()),
                    None => None,
                }
            }
        }

        impl HyprDataVec<$held> for $name {
            fn to_vec(self) -> Vec<$held> {
                self.held
            }
        }
    };

    (sing $name:ident,$kind:path,$held:ty,$c:literal) => {
        #[doc = $c]
        #[derive(Debug)]
        pub struct $name($held);

        #[async_trait]
        impl HyprData for $name {
            fn get() -> HResult<Self> {
                let data = call_hyprctl_data_cmd($kind);
                let deserialized: $held = serde_json::from_str(&data)?;
                Ok(Self(deserialized))
            }
            async fn get_async() -> HResult<Self> {
                let data = call_hyprctl_data_cmd_async($kind).await;
                let deserialized: $held = serde_json::from_str(&data)?;
                Ok(Self(deserialized))
            }
        }
    };

    (p $name:ident,$kind:path,$caller:expr,$held:ty,$c:literal) => {
        #[doc = $c]
        #[derive(Debug)]
        pub struct $name($held);

        #[async_trait]
        impl HyprData for $name {
            fn get() -> HResult<Self> {
                let data = call_hyprctl_data_cmd($kind);
                Ok(Self($caller(data)?))
            }
            async fn get_async() -> HResult<Self> {
                let data = call_hyprctl_data_cmd_async($kind).await;
                Ok(Self($caller(data)?))
            }
        }
    };
}

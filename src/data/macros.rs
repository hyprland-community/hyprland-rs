macro_rules! impl_on {
    ($name:ident) => {
        #[async_trait]
        impl HyprData for $name {
            fn get() -> $crate::Result<Self> {
                let data = call_hyprctl_data_cmd(DataCommands::$name);
                let deserialized: $name = serde_json::from_str(&data)?;
                Ok(deserialized)
            }
            async fn get_async() -> $crate::Result<Self> {
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
        pub struct $name(Vec<$held>);

        impl $name {
            /// Get the iterator by references of monitors.
            pub fn iter(&self) -> std::slice::Iter<$held> {
                self.0.iter()
            }

            /// Get the iterator by mutable references of monitors.
            pub fn iter_mut(&mut self) -> std::slice::IterMut<$held> {
                self.0.iter_mut()
            }
        }

        #[async_trait]
        impl HyprData for $name {
            fn get() -> $crate::Result<Self> {
                let data = call_hyprctl_data_cmd($kind);
                let deserialized: Vec<$held> = serde_json::from_str(&data)?;
                Ok(Self(deserialized))
            }
            async fn get_async() -> $crate::Result<Self> {
                let data = call_hyprctl_data_cmd_async($kind).await;
                let deserialized: Vec<$held> = serde_json::from_str(&data)?;
                Ok(Self(deserialized))
            }
        }

        impl IntoIterator for $name {
            type Item = $held;

            type IntoIter = std::vec::IntoIter<$held>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a> IntoIterator for &'a $name {
            type Item = &'a $held;
            type IntoIter = std::slice::Iter<'a, $held>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $name {
            type Item = &'a mut $held;
            type IntoIter = std::slice::IterMut<'a, $held>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }

        impl HyprDataVec<$held> for $name {
            fn to_vec(self) -> Vec<$held> {
                self.0
            }
        }
    };

    (sing $name:ident,$kind:path,$held:ty,$c:literal $(, iter_item = $it_it:ty)*) => {
        #[doc = $c]
        #[derive(Debug)]
        pub struct $name($held);

        impl $name {
            $(
                /// Get the iterator from the `held` Vec without having to move/clone data
                pub fn iter(&self) -> impl Iterator<Item = $it_it> {
                    self.0.iter()
                }
            )*
        }

        #[async_trait]
        impl HyprData for $name {
            fn get() -> $crate::Result<Self> {
                let data = call_hyprctl_data_cmd($kind);
                let deserialized: $held = serde_json::from_str(&data)?;
                Ok(Self(deserialized))
            }
            async fn get_async() -> $crate::Result<Self> {
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

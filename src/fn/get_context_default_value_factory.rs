use std::sync::Arc;

/// Context
/// @ providerName - Name of the context provider component tag
///
/// Get a default value factory that explicitly throws an error
pub fn get_context_default_value_factory(
    provider_name: &str,
) -> impl FnOnce(&str) -> Arc<dyn Fn() + Send + Sync> {
    let provider_name = provider_name.to_string();
    move |member_name: &str| {
        let member_name = member_name.to_string();
        Arc::new(move || {
            tracing::info!(
                "You must either init <{} /> or impl {}",
                provider_name,
                member_name
            );
        })
    }
}

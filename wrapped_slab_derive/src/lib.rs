use ::proc_macro_error::proc_macro_error;
use ::quote::{format_ident, quote};
use ::syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(WrappedSlab, attributes(multi_index))]
#[proc_macro_error]
pub fn wrapped_slab_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let element_name = input.ident;
    let element_vis = input.vis;

    // Generate the name of the MultiIndexMap
    let slab_name = format_ident!("{element_name}Slab");
    let vacant_entry_name = format_ident!("{element_name}VacantEntry");
    let key_name = format_ident!("{element_name}Key");

    let expanded = quote! {
        #[derive(Default)]
        #element_vis struct #slab_name(wrapped_slab::slab::Slab<#element_name>);

        #element_vis struct #vacant_entry_name<'a>(wrapped_slab::slab::VacantEntry<'a, #element_name>);

        #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        #element_vis struct #key_name(usize);

        impl<'a> #vacant_entry_name<'a> {
            pub fn key(&self) -> #key_name {
                #key_name(self.0.key())
            }

            pub fn insert(self, val: #element_name) -> &'a mut #element_name {
                self.0.insert(val)
            }
        }

        impl #slab_name {
            pub const fn new() -> Self {
                Self(wrapped_slab::slab::Slab::new())
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self(wrapped_slab::slab::Slab::with_capacity(capacity))
            }

            pub fn capacity(&self) -> usize {
                self.0.capacity()
            }

            pub fn reserve(&mut self, additional: usize) {
                self.0.reserve(additional)
            }

            pub fn reserve_exact(&mut self, additional: usize) {
                self.0.reserve_exact(additional)
            }

            pub fn shrink_to_fit(&mut self) {
                self.0.shrink_to_fit()
            }

            pub fn clear(&mut self) {
                self.0.clear()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn get(&self, key: #key_name) -> Option<&#element_name> {
                self.0.get(key.0)
            }

            pub fn get_mut(&mut self, key: #key_name) -> Option<&mut #element_name> {
                self.0.get_mut(key.0)
            }

            pub fn get2_mut(&mut self, key1: #key_name, key2: #key_name) -> Option<(&mut #element_name, &mut #element_name)> {
                self.0.get2_mut(key1.0, key2.0)
            }

            pub unsafe fn get_unchecked(&self, key: #key_name) -> &#element_name {
                self.0.get_unchecked(key.0)
            }

            pub unsafe fn get_unchecked_mut(&mut self, key: #key_name) -> &mut #element_name {
                self.0.get_unchecked_mut(key.0)
            }

            pub unsafe fn get2_unchecked_mut(&mut self, key1: #key_name, key2: #key_name) -> (&mut #element_name, &mut #element_name) {
                self.0.get2_unchecked_mut(key1.0, key2.0)
            }

            pub fn key_of(&self, present_element: &#element_name) -> #key_name {
                #key_name(self.0.key_of(present_element))
            }

            pub fn insert(&mut self, val: #element_name) -> #key_name {
                #key_name(self.0.insert(val))
            }

            pub fn vacant_key(&self) -> #key_name {
                #key_name(self.0.vacant_key())
            }

            pub fn vacant_entry(&mut self) -> #vacant_entry_name<'_> {
                #vacant_entry_name(self.0.vacant_entry())
            }

            pub fn try_remove(&mut self, key: #key_name) -> Option<#element_name> {
                self.0.try_remove(key.0)
            }

            pub fn remove(&mut self, key: #key_name) -> #element_name {
                self.0.remove(key.0)
            }

            pub fn contains(&self, key: #key_name) -> bool {
                self.0.contains(key.0)
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

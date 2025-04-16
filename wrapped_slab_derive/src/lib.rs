use ::proc_macro_error::proc_macro_error;
use ::quote::{format_ident, quote};
use ::syn::{parse_macro_input, DeriveInput};
use proc_macro2::Span;
use syn::{Lifetime, LifetimeDef};

#[proc_macro_derive(WrappedSlab, attributes(multi_index))]
#[proc_macro_error]
pub fn wrapped_slab_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let element_name = input.ident;
    let element_vis = input.vis;

    let (impls, types, where_clause) = input.generics.split_for_impl();

    let mut extra_generics = input.generics.clone();
    // let (impls, types, where_clause) = input.generics.split_for_impl();
    let lf = Lifetime::new("'a", Span::call_site());
    extra_generics
        .params
        .push(syn::GenericParam::Lifetime(LifetimeDef::new(lf)));
    let (impls_extra, types_extra, where_clause_extra) = extra_generics.split_for_impl();

    let mut empty_generics = input.generics.clone();
    // let (impls, types, where_clause) = input.generics.split_for_impl();
    let lf = Lifetime::new("'_", Span::call_site());
    empty_generics
        .params
        .push(syn::GenericParam::Lifetime(LifetimeDef::new(lf)));
    let (_, types_empty, _) = empty_generics.split_for_impl();

    // let element_type_generics = input.generics.type_params();

    // Generate the name of the MultiIndexMap
    let slab_name = format_ident!("{element_name}Slab");
    let vacant_entry_name = format_ident!("{element_name}VacantEntry");
    let key_name = format_ident!("{element_name}Key");
    let iter_name = format_ident!("{element_name}Iter");
    let iter_mut_name = format_ident!("{element_name}IterMut");
    let into_iter_name = format_ident!("{element_name}IntoIter");

    let expanded = quote! {
        #[derive(Default)]
        #element_vis struct #slab_name #types(wrapped_slab::slab::Slab<#element_name #types>);

        #element_vis struct #vacant_entry_name #types_extra(wrapped_slab::slab::VacantEntry<'a, #element_name #types>);

        #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
        #element_vis struct #key_name(usize);

        impl ::core::fmt::Display for #key_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl #impls_extra #vacant_entry_name #types_extra #where_clause_extra {
            pub fn key(&self) -> #key_name {
                #key_name(self.0.key())
            }

            pub fn insert(self, val: #element_name #types) -> &'a mut #element_name #types {
                self.0.insert(val)
            }
        }

        #element_vis struct #iter_name #types_extra (wrapped_slab::slab::Iter<'a, #element_name #types>);

        #element_vis struct #iter_mut_name #types_extra (wrapped_slab::slab::IterMut<'a, #element_name #types>);

        #element_vis struct #into_iter_name #types (wrapped_slab::slab::IntoIter<#element_name #types>);

        impl #impls_extra Iterator for #iter_name #types_extra #where_clause_extra {
            type Item = (#key_name, &'a #element_name #types);

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|(key, val)| (#key_name(key), val))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }

        impl #impls DoubleEndedIterator for #iter_name #types_empty #where_clause {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back().map(|(key, val)| (#key_name(key), val))
            }
        }

        impl #impls ExactSizeIterator for #iter_name #types_empty #where_clause {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl #impls ::core::iter::FusedIterator for #iter_name #types_empty #where_clause {}

        impl #impls_extra Iterator for #iter_mut_name #types_extra #where_clause_extra {
            type Item = (#key_name, &'a mut #element_name #types);

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|(key, val)| (#key_name(key), val))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }

        impl #impls DoubleEndedIterator for #iter_mut_name #types_empty #where_clause {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back().map(|(key, val)| (#key_name(key), val))
            }
        }

        impl #impls ExactSizeIterator for #iter_mut_name #types_empty #where_clause {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl #impls ::core::iter::FusedIterator for #iter_mut_name #types_empty #where_clause {}

        impl #impls Iterator for #into_iter_name #types #where_clause {
            type Item = (#key_name, #element_name #types);

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map(|(key, val)| (#key_name(key), val))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }

        impl #impls DoubleEndedIterator for #into_iter_name #types #where_clause {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back().map(|(key, val)| (#key_name(key), val))
            }
        }

        impl #impls ExactSizeIterator for #into_iter_name #types {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl #impls ::core::iter::FusedIterator for #into_iter_name #types #where_clause {}

        impl #impls_extra IntoIterator for &'a #slab_name #types #where_clause_extra {
            type Item = (#key_name, &'a #element_name #types);
            type IntoIter = #iter_name #types_extra;

            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl #impls_extra IntoIterator for &'a mut #slab_name #types #where_clause_extra {
            type Item = (#key_name, &'a mut #element_name #types);
            type IntoIter = #iter_mut_name #types_extra;

            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }

        impl #impls IntoIterator for #slab_name #types #where_clause {
            type Item = (#key_name, #element_name #types);
            type IntoIter = #into_iter_name #types;

            fn into_iter(self) -> Self::IntoIter {
                #into_iter_name(self.0.into_iter())
            }
        }

        impl #impls ::core::ops::Index<#key_name> for #slab_name #types #where_clause {
            type Output = #element_name #types;

            fn index(&self, key: #key_name) -> &#element_name #types {
                self.0.index(key.0)
            }
        }

        impl #impls ::core::ops::IndexMut<#key_name> for #slab_name #types #where_clause {
            fn index_mut(&mut self, key: #key_name) -> &mut #element_name #types {
                self.0.index_mut(key.0)
            }
        }

        impl #impls #slab_name #types #where_clause {
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

            pub fn get(&self, key: #key_name) -> Option<&#element_name #types> {
                self.0.get(key.0)
            }

            pub fn get_mut(&mut self, key: #key_name) -> Option<&mut #element_name #types> {
                self.0.get_mut(key.0)
            }

            pub fn get2_mut(&mut self, key1: #key_name, key2: #key_name) -> Option<(&mut #element_name #types, &mut #element_name #types)> {
                self.0.get2_mut(key1.0, key2.0)
            }

            pub unsafe fn get_unchecked(&self, key: #key_name) -> &#element_name #types {
                self.0.get_unchecked(key.0)
            }

            pub unsafe fn get_unchecked_mut(&mut self, key: #key_name) -> &mut #element_name #types {
                self.0.get_unchecked_mut(key.0)
            }

            pub unsafe fn get2_unchecked_mut(&mut self, key1: #key_name, key2: #key_name) -> (&mut #element_name #types, &mut #element_name #types) {
                self.0.get2_unchecked_mut(key1.0, key2.0)
            }

            pub fn key_of(&self, present_element: &#element_name #types) -> #key_name {
                #key_name(self.0.key_of(present_element))
            }

            pub fn insert(&mut self, val: #element_name #types) -> #key_name {
                #key_name(self.0.insert(val))
            }

            pub fn vacant_key(&self) -> #key_name {
                #key_name(self.0.vacant_key())
            }

            pub fn vacant_entry(&mut self) -> #vacant_entry_name #types_empty {
                #vacant_entry_name(self.0.vacant_entry())
            }

            pub fn try_remove(&mut self, key: #key_name) -> Option<#element_name #types> {
                self.0.try_remove(key.0)
            }

            pub fn remove(&mut self, key: #key_name) -> #element_name #types {
                self.0.remove(key.0)
            }

            pub fn contains(&self, key: #key_name) -> bool {
                self.0.contains(key.0)
            }

            pub fn iter(&self) -> #iter_name #types_empty {
                #iter_name(self.0.iter())
            }

            pub fn iter_mut(&mut self) -> #iter_mut_name #types_empty {
                #iter_mut_name(self.0.iter_mut())
            }

            pub fn drain(&mut self) -> wrapped_slab::slab::Drain<'_, #element_name #types> {
                self.0.drain()
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

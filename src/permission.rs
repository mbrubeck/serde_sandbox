#![feature(custom_derive)]
#![feature(libc)]

use json_types::{Point, Permission};
use std::slice;
use std::ffi::CStr;
use std::panic;
use std::str;
use libc::{size_t, c_char};
use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hash};
use seahash::SeaHasher;

type SeaHashSet<K> = HashSet<K, BuildHasherDefault<SeaHasher>>;

static PART_DELIMETER: &'static str = ":";
static SUBPART_DELIMETER: &'static str = ",";


impl<'a> Permission<&'a str> {
    fn new(wildcard_perm: &str) -> Permission<&str> {
        let mut perm = Permission {
            domain: None,
            actions: None,
            targets: None,
        };
        perm.init_parts(wildcard_perm);
        perm
    }

    fn part_from_str(s: Option<&str>) -> Option<SeaHashSet<&str>> {
        match s {
            Some("") | None => None,
            Some(s) => {
                let mut set = SeaHashSet::default();
                for rule in s.split(SUBPART_DELIMETER).map(str::trim) {
                    if rule == "*" { return None }
                    set.insert(rule);
                }
                Some(set)
            }
        }

    }

    fn init_parts(&mut self, wildcard_perm: &'a str) {
        let mut iter = wildcard_perm.split(PART_DELIMETER).map(str::trim);

        self.domain = match iter.next() {
            Some("") | Some("*") | None => None,
            domain => domain
        };
        self.actions = Permission::part_from_str(iter.next());
        self.targets = Permission::part_from_str(iter.next());
    }

    fn implies_from_str(&self, wildcard_permission: &str) -> bool {
        let permission = Permission::new(wildcard_permission);
        self.implies_from_perm(&permission)
    }
}

impl<T> Permission<T> where T: Hash + Eq {
    fn implies_from_perm(&self, permission: &Permission<T>) -> bool {
        match (self.domain.as_ref(), permission.domain.as_ref()) {
            (Some(ref my_domain), Some(ref other_domain)) if my_domain != other_domain => {
                return false;
            }
            _ => {}
        }

        match (self.actions.as_ref(), permission.actions.as_ref()) {
            (Some(my_set), Some(other_set)) if !my_set.is_superset(other_set) => {
                return false;
            }
            (None, Some(_)) => return false,
            _ => {}
        }

        match (self.actions.as_ref(), permission.actions.as_ref()) {
            (Some(my_set), Some(other_set)) if !my_set.is_superset(other_set) => {
                return false;
            }
            (None, Some(_)) => return false,
            _ => {}
        }

        return true;
    }
}

#[no_mangle]
pub extern fn is_permitted_from_str(required_perm: *const c_char, assigned_perms: *const *const c_char, assigned_perms_len: size_t) -> i32 {

    let res = panic::catch_unwind(|| {
        let ffi_required_permission = unsafe {CStr::from_ptr(required_perm)};
        let required_permission = match ffi_required_permission.to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };

        let perms = unsafe {slice::from_raw_parts(assigned_perms, assigned_perms_len as usize)};
        let assigned_permissions = perms.iter()
            .map(|&p| unsafe { CStr::from_ptr(p) })
            .map(|cs| cs.to_bytes())
            .map(|bs| str::from_utf8(bs).unwrap_or_else(|_| ""));

        _is_permitted_from_str(required_permission, assigned_permissions)
    });

    match res {
           Ok(rc) => rc,
           Err(_) => -1,
       }
}

fn _is_permitted_from_str<'a, I>(required_perm: &str, assigned_perms: I) -> i32
    where I: IntoIterator<Item=&'a str>
{
    let required_permission = Permission::new(&required_perm);

    for assigned in assigned_perms {
        let assigned_permission = Permission::new(assigned);
        if assigned_permission.implies_from_perm(&required_permission){
            return 1;
        }
    }
    return 0;
}

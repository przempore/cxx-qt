// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We only use pointers to C types here so we don't care what the types actually contain.
// Unfortunately giving names to the C types here doesn't provide any type safety during
// pointer type casts, it only looks better than using c_void.
#![allow(improper_ctypes)]

#[repr(C)]
struct QPtr {}

#[repr(C)]
pub struct CxxQObject {}

extern "C" {
    fn cxx_qt_update_requester_new(qobject_ptr: *mut CxxQObject) -> *mut QPtr;
    fn cxx_qt_update_requester_drop(s: *mut QPtr);
    fn cxx_qt_update_requester_request_update(s: *const QPtr) -> bool;
    fn cxx_qt_update_requester_clone(s: *const QPtr) -> *mut QPtr;
}

#[derive(Debug)]
pub struct UpdateRequester {
    requester: *mut QPtr,
}

impl UpdateRequester {
    /// Safety:
    ///
    /// You may only call this if qobject_ptr is a valid pointer to a
    /// C++ object that derives from CxxQObject. Once constructed, you
    /// may destroy the object as we use QPointer to handle such events.
    pub unsafe fn new(qobject_ptr: *mut CxxQObject) -> Self {
        Self {
            requester: cxx_qt_update_requester_new(qobject_ptr),
        }
    }

    pub fn request_update(&self) -> bool {
        // Safety:
        //
        // The caller should ensure that a valid pointer is called during new,
        // which is why only that function is marked unsafe.
        unsafe { cxx_qt_update_requester_request_update(self.requester) }
    }
}

impl Drop for UpdateRequester {
    fn drop(&mut self) {
        // Safety:
        //
        // The caller should ensure that a valid pointer is called during new,
        // which is why only that function is marked unsafe.
        unsafe {
            cxx_qt_update_requester_drop(self.requester);
        }
    }
}

impl Clone for UpdateRequester {
    fn clone(&self) -> UpdateRequester {
        // Safety:
        //
        // The caller should ensure that a valid pointer is called during new,
        // which is why only that function is marked unsafe.
        unsafe {
            Self {
                requester: cxx_qt_update_requester_clone(self.requester),
            }
        }
    }
}

// Safety:
//
// The underlying C++ class has been designed to be thread safe and we only
// store a pointer to it which is valid from any thread.
unsafe impl Send for UpdateRequester {}
unsafe impl Sync for UpdateRequester {}
unsafe impl Send for QPtr {}
unsafe impl Sync for QPtr {}
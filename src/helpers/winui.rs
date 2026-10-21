#[allow(warnings)]
#[allow(nonstandard_style)]
#[allow(non_snake_case)]

pub mod Microsoft {
    pub mod UI {
        pub mod Xaml {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Application(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                Application,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl Application {
                pub fn Resources(&self) -> windows_core::Result<ResourceDictionary> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Resources)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<ResourceDictionary>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).SetResources)(
                            windows_core::Interface::as_raw(self),
                            value.param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn RequestedTheme(&self) -> windows_core::Result<ApplicationTheme> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).RequestedTheme)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn SetRequestedTheme(
                    &self,
                    value: ApplicationTheme,
                ) -> windows_core::Result<()> {
                    unsafe {
                        (windows_core::Interface::vtable(self).SetRequestedTheme)(
                            windows_core::Interface::as_raw(self),
                            value,
                        )
                        .ok()
                    }
                }
                pub fn new() -> windows_core::Result<Self> {
                    Self::IApplicationFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn compose<T>(compose: T) -> windows_core::Result<Self>
                where
                    T: windows_core::Compose,
                {
                    Self::IApplicationFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                pub fn Current() -> windows_core::Result<Self> {
                    Self::IApplicationStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Current)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn Start<P0>(callback: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<ApplicationInitializationCallback>,
                {
                    Self::IApplicationStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).Start)(
                            windows_core::Interface::as_raw(this),
                            callback.param().abi(),
                        )
                        .ok()
                    })
                }
                pub fn LoadComponent<P0, P1>(
                    component: P0,
                    resourcelocator: P1,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                    P1: windows_core::Param<super::super::super::Windows::Foundation::Uri>,
                {
                    Self::IApplicationStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).LoadComponent)(
                            windows_core::Interface::as_raw(this),
                            component.param().abi(),
                            resourcelocator.param().abi(),
                        )
                        .ok()
                    })
                }
                pub fn LoadComponentWithResourceLocation<P0, P1>(
                    component: P0,
                    resourcelocator: P1,
                    componentresourcelocation: Controls::Primitives::ComponentResourceLocation,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                    P1: windows_core::Param<super::super::super::Windows::Foundation::Uri>,
                {
                    Self::IApplicationStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).LoadComponentWithResourceLocation)(
                            windows_core::Interface::as_raw(this),
                            component.param().abi(),
                            resourcelocator.param().abi(),
                            componentresourcelocation,
                        )
                        .ok()
                    })
                }
                fn IApplicationFactory<
                    R,
                    F: FnOnce(&IApplicationFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        Application,
                        IApplicationFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IApplicationStatics<
                    R,
                    F: FnOnce(&IApplicationStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        Application,
                        IApplicationStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for Application {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IApplication>();
            }
            unsafe impl windows_core::Interface for Application {
                type Vtable = <IApplication as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IApplication as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Application {
                const NAME: &'static str = "Microsoft.UI.Xaml.Application";
            }
            unsafe impl Send for Application {}
            unsafe impl Sync for Application {}
            windows_core::imp::define_interface!(
                ApplicationInitializationCallback,
                ApplicationInitializationCallback_Vtbl,
                0xd8eef1c9_1234_56f1_9963_45dd9c80a661
            );
            impl windows_core::RuntimeType for ApplicationInitializationCallback {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl ApplicationInitializationCallback {
                pub fn new<
                    F: Fn(
                            windows_core::Ref<ApplicationInitializationCallbackParams>,
                        ) -> windows_core::Result<()>
                        + Send
                        + 'static,
                >(
                    invoke: F,
                ) -> Self {
                    let com = windows_core::imp::DelegateBox::<Self, F>::new(
                        &ApplicationInitializationCallbackBox::<F>::VTABLE,
                        invoke,
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                }
                pub fn Invoke<P0>(&self, p: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<ApplicationInitializationCallbackParams>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).Invoke)(
                            windows_core::Interface::as_raw(self),
                            p.param().abi(),
                        )
                        .ok()
                    }
                }
            }
            #[repr(C)]
            pub struct ApplicationInitializationCallback_Vtbl {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(
                    this: *mut core::ffi::c_void,
                    p: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            struct ApplicationInitializationCallbackBox<
                F: Fn(
                        windows_core::Ref<ApplicationInitializationCallbackParams>,
                    ) -> windows_core::Result<()>
                    + Send
                    + 'static,
            >(core::marker::PhantomData<(fn() -> F,)>);
            impl<
                F: Fn(
                        windows_core::Ref<ApplicationInitializationCallbackParams>,
                    ) -> windows_core::Result<()>
                    + Send
                    + 'static,
            > ApplicationInitializationCallbackBox<F>
            {
                const VTABLE: ApplicationInitializationCallback_Vtbl =
                    ApplicationInitializationCallback_Vtbl {
                        base__: windows_core::IUnknown_Vtbl {
                            QueryInterface: windows_core::imp::DelegateBox::<
                                ApplicationInitializationCallback,
                                F,
                            >::QueryInterface,
                            AddRef: windows_core::imp::DelegateBox::<
                                ApplicationInitializationCallback,
                                F,
                            >::AddRef,
                            Release: windows_core::imp::DelegateBox::<
                                ApplicationInitializationCallback,
                                F,
                            >::Release,
                        },
                        Invoke: Self::Invoke,
                    };
                unsafe extern "system" fn Invoke(
                    this: *mut core::ffi::c_void,
                    p: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void
                            as *mut windows_core::imp::DelegateBox<
                                ApplicationInitializationCallback,
                                F,
                            >);
                        (this.invoke)(core::mem::transmute_copy(&p)).into()
                    }
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ApplicationInitializationCallbackParams(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                ApplicationInitializationCallbackParams,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl windows_core::RuntimeType for ApplicationInitializationCallbackParams {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<
                        Self,
                        IApplicationInitializationCallbackParams,
                    >();
            }
            unsafe impl windows_core::Interface for ApplicationInitializationCallbackParams {
                type Vtable =
                    <IApplicationInitializationCallbackParams as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IApplicationInitializationCallbackParams as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ApplicationInitializationCallbackParams {
                const NAME: &'static str =
                    "Microsoft.UI.Xaml.ApplicationInitializationCallbackParams";
            }
            unsafe impl Send for ApplicationInitializationCallbackParams {}
            unsafe impl Sync for ApplicationInitializationCallbackParams {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct ApplicationTheme(pub i32);
            impl ApplicationTheme {
                pub const Light: Self = Self(0);
                pub const Dark: Self = Self(1);
            }
            impl windows_core::imp::TypeKind for ApplicationTheme {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for ApplicationTheme {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"enum(Microsoft.UI.Xaml.ApplicationTheme;i4)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.ApplicationTheme",
                    );
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct CornerRadius {
                pub TopLeft: f64,
                pub TopRight: f64,
                pub BottomRight: f64,
                pub BottomLeft: f64,
            }
            impl windows_core::imp::TypeKind for CornerRadius {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for CornerRadius {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"struct(Microsoft.UI.Xaml.CornerRadius;f8;f8;f8;f8)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.CornerRadius");
            }
            windows_core::imp::define_interface!(
                CreateDefaultValueCallback,
                CreateDefaultValueCallback_Vtbl,
                0x7f808c05_2ac4_5ad9_ac8a_26890333d81e
            );
            impl windows_core::RuntimeType for CreateDefaultValueCallback {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl CreateDefaultValueCallback {
                pub fn new<
                    F: Fn() -> windows_core::Result<windows_core::IInspectable> + Send + 'static,
                >(
                    invoke: F,
                ) -> Self {
                    let com = windows_core::imp::DelegateBox::<Self, F>::new(
                        &CreateDefaultValueCallbackBox::<F>::VTABLE,
                        invoke,
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                }
                pub fn Invoke(&self) -> windows_core::Result<windows_core::IInspectable> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Invoke)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            #[repr(C)]
            pub struct CreateDefaultValueCallback_Vtbl {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(
                    this: *mut core::ffi::c_void,
                    result__: *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            struct CreateDefaultValueCallbackBox<
                F: Fn() -> windows_core::Result<windows_core::IInspectable> + Send + 'static,
            >(core::marker::PhantomData<(fn() -> F,)>);
            impl<F: Fn() -> windows_core::Result<windows_core::IInspectable> + Send + 'static>
                CreateDefaultValueCallbackBox<F>
            {
                const VTABLE: CreateDefaultValueCallback_Vtbl = CreateDefaultValueCallback_Vtbl {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface: windows_core::imp::DelegateBox::<
                            CreateDefaultValueCallback,
                            F,
                        >::QueryInterface,
                        AddRef:
                            windows_core::imp::DelegateBox::<CreateDefaultValueCallback, F>::AddRef,
                        Release:
                            windows_core::imp::DelegateBox::<CreateDefaultValueCallback, F>::Release,
                    },
                    Invoke: Self::Invoke,
                };
                unsafe extern "system" fn Invoke(
                    this: *mut core::ffi::c_void,
                    result__: *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void
                            as *mut windows_core::imp::DelegateBox<CreateDefaultValueCallback, F>);
                        match (this.invoke)() {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                core::mem::forget(ok__);
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DependencyObject(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                DependencyObject,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl DependencyObject {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IDependencyObjectFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn compose<T>(compose: T) -> windows_core::Result<Self>
                where
                    T: windows_core::Compose,
                {
                    Self::IDependencyObjectFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                fn IDependencyObjectFactory<
                    R,
                    F: FnOnce(&IDependencyObjectFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        DependencyObject,
                        IDependencyObjectFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for DependencyObject {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
            }
            unsafe impl windows_core::Interface for DependencyObject {
                type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DependencyObject {
                const NAME: &'static str = "Microsoft.UI.Xaml.DependencyObject";
            }
            unsafe impl Send for DependencyObject {}
            unsafe impl Sync for DependencyObject {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DependencyProperty(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                DependencyProperty,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl DependencyProperty {
                pub fn UnsetValue() -> windows_core::Result<windows_core::IInspectable> {
                    Self::IDependencyPropertyStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).UnsetValue)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn Register<P3>(
                    name: &windows_core::HSTRING,
                    propertytype: &super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    ownertype: &super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    typemetadata: P3,
                ) -> windows_core::Result<Self>
                where
                    P3: windows_core::Param<PropertyMetadata>,
                {
                    Self::IDependencyPropertyStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Register)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(name),
                            core::mem::transmute_copy(propertytype),
                            core::mem::transmute_copy(ownertype),
                            typemetadata.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn RegisterAttached<P3>(
                    name: &windows_core::HSTRING,
                    propertytype: &super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    ownertype: &super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    defaultmetadata: P3,
                ) -> windows_core::Result<Self>
                where
                    P3: windows_core::Param<PropertyMetadata>,
                {
                    Self::IDependencyPropertyStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).RegisterAttached)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(name),
                            core::mem::transmute_copy(propertytype),
                            core::mem::transmute_copy(ownertype),
                            defaultmetadata.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                fn IDependencyPropertyStatics<
                    R,
                    F: FnOnce(&IDependencyPropertyStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        DependencyProperty,
                        IDependencyPropertyStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for DependencyProperty {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IDependencyProperty>();
            }
            unsafe impl windows_core::Interface for DependencyProperty {
                type Vtable = <IDependencyProperty as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IDependencyProperty as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DependencyProperty {
                const NAME: &'static str = "Microsoft.UI.Xaml.DependencyProperty";
            }
            unsafe impl Send for DependencyProperty {}
            unsafe impl Sync for DependencyProperty {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DependencyPropertyChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                DependencyPropertyChangedEventArgs,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl windows_core::RuntimeType for DependencyPropertyChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<
                        Self,
                        IDependencyPropertyChangedEventArgs,
                    >();
            }
            unsafe impl windows_core::Interface for DependencyPropertyChangedEventArgs {
                type Vtable =
                    <IDependencyPropertyChangedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IDependencyPropertyChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DependencyPropertyChangedEventArgs {
                const NAME: &'static str = "Microsoft.UI.Xaml.DependencyPropertyChangedEventArgs";
            }
            unsafe impl Send for DependencyPropertyChangedEventArgs {}
            unsafe impl Sync for DependencyPropertyChangedEventArgs {}
            windows_core::imp::define_interface!(
                IApplication,
                IApplication_Vtbl,
                0x06a8f4e7_1146_55af_820d_ebd55643b021
            );
            impl windows_core::RuntimeType for IApplication {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplication");
            }
            #[repr(C)]
            pub struct IApplication_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Resources: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetResources: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                DebugSettings: usize,
                pub RequestedTheme: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut ApplicationTheme,
                )
                    -> windows_core::HRESULT,
                pub SetRequestedTheme: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    ApplicationTheme,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IApplicationFactory,
                IApplicationFactory_Vtbl,
                0x9fd96657_5294_5a65_a1db_4fea143597da
            );
            impl windows_core::RuntimeType for IApplicationFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IApplicationFactory",
                    );
            }
            #[repr(C)]
            pub struct IApplicationFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IApplicationInitializationCallbackParams,
                IApplicationInitializationCallbackParams_Vtbl,
                0x1b1906ea_5b7b_5876_81ab_7c2281ac3d20
            );
            impl windows_core::RuntimeType for IApplicationInitializationCallbackParams {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IApplicationInitializationCallbackParams",
                    );
            }
            #[repr(C)]
            pub struct IApplicationInitializationCallbackParams_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IApplicationStatics,
                IApplicationStatics_Vtbl,
                0x4e0d09f5_4358_512c_a987_503b52848e95
            );
            impl windows_core::RuntimeType for IApplicationStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IApplicationStatics",
                    );
            }
            #[repr(C)]
            pub struct IApplicationStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Current: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Start: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub LoadComponent: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub LoadComponentWithResourceLocation:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        Controls::Primitives::ComponentResourceLocation,
                    ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDependencyObject,
                IDependencyObject_Vtbl,
                0xe7beaee7_160e_50f7_8789_d63463f979fa
            );
            impl windows_core::RuntimeType for IDependencyObject {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IDependencyObject",
                    );
            }
            #[repr(C)]
            pub struct IDependencyObject_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IDependencyObjectFactory,
                IDependencyObjectFactory_Vtbl,
                0x936b614c_475f_5d7d_b3f7_bf1fbea28126
            );
            impl windows_core::RuntimeType for IDependencyObjectFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IDependencyObjectFactory",
                    );
            }
            #[repr(C)]
            pub struct IDependencyObjectFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDependencyProperty,
                IDependencyProperty_Vtbl,
                0x960eab49_9672_58a0_995b_3a42e5ea6278
            );
            impl windows_core::RuntimeType for IDependencyProperty {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IDependencyProperty",
                    );
            }
            #[repr(C)]
            pub struct IDependencyProperty_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IDependencyPropertyChangedEventArgs,
                IDependencyPropertyChangedEventArgs_Vtbl,
                0x84ead020_7849_5e98_8030_488a80d164ec
            );
            impl windows_core::RuntimeType for IDependencyPropertyChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IDependencyPropertyChangedEventArgs",
                    );
            }
            #[repr(C)]
            pub struct IDependencyPropertyChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IDependencyPropertyStatics,
                IDependencyPropertyStatics_Vtbl,
                0x61ddc651_0383_5d6f_98ce_5c046aaaaa8f
            );
            impl windows_core::RuntimeType for IDependencyPropertyStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IDependencyPropertyStatics",
                    );
            }
            #[repr(C)]
            pub struct IDependencyPropertyStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub UnsetValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub Register: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    core::mem::MaybeUninit<
                        super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    >,
                    core::mem::MaybeUninit<
                        super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    >,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub RegisterAttached: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    core::mem::MaybeUninit<
                        super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    >,
                    core::mem::MaybeUninit<
                        super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    >,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPropertyMetadata,
                IPropertyMetadata_Vtbl,
                0xb3644425_9464_5434_b0ae_aff8d3159fe1
            );
            impl windows_core::RuntimeType for IPropertyMetadata {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IPropertyMetadata",
                    );
            }
            #[repr(C)]
            pub struct IPropertyMetadata_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IPropertyMetadataFactory,
                IPropertyMetadataFactory_Vtbl,
                0x9f420906_111a_5465_91ee_bed14b3e7fec
            );
            impl windows_core::RuntimeType for IPropertyMetadataFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IPropertyMetadataFactory",
                    );
            }
            #[repr(C)]
            pub struct IPropertyMetadataFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstanceWithDefaultValue:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub CreateInstanceWithDefaultValueAndCallback:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IPropertyMetadataStatics,
                IPropertyMetadataStatics_Vtbl,
                0x37b8add4_7a4a_5cf7_a174_235182cd082e
            );
            impl windows_core::RuntimeType for IPropertyMetadataStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IPropertyMetadataStatics",
                    );
            }
            #[repr(C)]
            pub struct IPropertyMetadataStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateWithDefaultValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub CreateWithDefaultValueAndCallback:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                pub CreateWithFactory: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub CreateWithFactoryAndCallback:
                    unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IResourceDictionary,
                IResourceDictionary_Vtbl,
                0x1b690975_a710_5783_a6e1_15836f6186c2
            );
            impl windows_core::RuntimeType for IResourceDictionary {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IResourceDictionary",
                    );
            }
            #[repr(C)]
            pub struct IResourceDictionary_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                Source: usize,
                SetSource: usize,
                pub MergedDictionaries: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
                pub ThemeDictionaries: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IResourceDictionaryFactory,
                IResourceDictionaryFactory_Vtbl,
                0xea22a48f_ab71_56f6_a392_d82310c8aa7b
            );
            impl windows_core::RuntimeType for IResourceDictionaryFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Microsoft.UI.Xaml.IResourceDictionaryFactory",
                    );
            }
            #[repr(C)]
            pub struct IResourceDictionaryFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                )
                    -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                PropertyChangedCallback,
                PropertyChangedCallback_Vtbl,
                0x5fd9243a_2422_53c9_8d6f_f1ba1a0bba9a
            );
            impl windows_core::RuntimeType for PropertyChangedCallback {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl PropertyChangedCallback {
                pub fn new<
                    F: Fn(
                            windows_core::Ref<DependencyObject>,
                            windows_core::Ref<DependencyPropertyChangedEventArgs>,
                        ) -> windows_core::Result<()>
                        + Send
                        + 'static,
                >(
                    invoke: F,
                ) -> Self {
                    let com = windows_core::imp::DelegateBox::<Self, F>::new(
                        &PropertyChangedCallbackBox::<F>::VTABLE,
                        invoke,
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                }
                pub fn Invoke<P0, P1>(&self, d: P0, e: P1) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<DependencyObject>,
                    P1: windows_core::Param<DependencyPropertyChangedEventArgs>,
                {
                    unsafe {
                        (windows_core::Interface::vtable(self).Invoke)(
                            windows_core::Interface::as_raw(self),
                            d.param().abi(),
                            e.param().abi(),
                        )
                        .ok()
                    }
                }
            }
            #[repr(C)]
            pub struct PropertyChangedCallback_Vtbl {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(
                    this: *mut core::ffi::c_void,
                    d: *mut core::ffi::c_void,
                    e: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            struct PropertyChangedCallbackBox<
                F: Fn(
                        windows_core::Ref<DependencyObject>,
                        windows_core::Ref<DependencyPropertyChangedEventArgs>,
                    ) -> windows_core::Result<()>
                    + Send
                    + 'static,
            >(core::marker::PhantomData<(fn() -> F,)>);
            impl<
                F: Fn(
                        windows_core::Ref<DependencyObject>,
                        windows_core::Ref<DependencyPropertyChangedEventArgs>,
                    ) -> windows_core::Result<()>
                    + Send
                    + 'static,
            > PropertyChangedCallbackBox<F>
            {
                const VTABLE : PropertyChangedCallback_Vtbl < > = PropertyChangedCallback_Vtbl::< > { base__ : windows_core::IUnknown_Vtbl { QueryInterface : windows_core::imp::DelegateBox::< PropertyChangedCallback , F >::QueryInterface , AddRef : windows_core::imp::DelegateBox::< PropertyChangedCallback , F >::AddRef , Release : windows_core::imp::DelegateBox::< PropertyChangedCallback , F >::Release , } , Invoke : Self::Invoke , } ;
                unsafe extern "system" fn Invoke(
                    this: *mut core::ffi::c_void,
                    d: *mut core::ffi::c_void,
                    e: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void
                            as *mut windows_core::imp::DelegateBox<PropertyChangedCallback, F>);
                        (this.invoke)(core::mem::transmute_copy(&d), core::mem::transmute_copy(&e))
                            .into()
                    }
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PropertyMetadata(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                PropertyMetadata,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl PropertyMetadata {
                pub fn CreateInstanceWithDefaultValue<P0>(
                    defaultvalue: P0,
                ) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                {
                    Self::IPropertyMetadataFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstanceWithDefaultValue)(
                            windows_core::Interface::as_raw(this),
                            defaultvalue.param().abi(),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn CreateInstanceWithDefaultValue_compose<P0, T>(
                    defaultvalue: P0,
                    compose: T,
                ) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                    T: windows_core::Compose,
                {
                    Self::IPropertyMetadataFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstanceWithDefaultValue)(
                            windows_core::Interface::as_raw(this),
                            defaultvalue.param().abi(),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                pub fn CreateInstanceWithDefaultValueAndCallback<P0, P1>(
                    defaultvalue: P0,
                    propertychangedcallback: P1,
                ) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                    P1: windows_core::Param<PropertyChangedCallback>,
                {
                    Self::IPropertyMetadataFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CreateInstanceWithDefaultValueAndCallback)(
                            windows_core::Interface::as_raw(this),
                            defaultvalue.param().abi(),
                            propertychangedcallback.param().abi(),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn CreateInstanceWithDefaultValueAndCallback_compose<P0, P1, T>(
                    defaultvalue: P0,
                    propertychangedcallback: P1,
                    compose: T,
                ) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                    P1: windows_core::Param<PropertyChangedCallback>,
                    T: windows_core::Compose,
                {
                    Self::IPropertyMetadataFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this)
                            .CreateInstanceWithDefaultValueAndCallback)(
                            windows_core::Interface::as_raw(this),
                            defaultvalue.param().abi(),
                            propertychangedcallback.param().abi(),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                pub fn CreateWithDefaultValue<P0>(defaultvalue: P0) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                {
                    Self::IPropertyMetadataStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateWithDefaultValue)(
                            windows_core::Interface::as_raw(this),
                            defaultvalue.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn CreateWithDefaultValueAndCallback<P0, P1>(
                    defaultvalue: P0,
                    propertychangedcallback: P1,
                ) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                    P1: windows_core::Param<PropertyChangedCallback>,
                {
                    Self::IPropertyMetadataStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateWithDefaultValueAndCallback)(
                            windows_core::Interface::as_raw(this),
                            defaultvalue.param().abi(),
                            propertychangedcallback.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn CreateWithFactory<P0>(
                    createdefaultvaluecallback: P0,
                ) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<CreateDefaultValueCallback>,
                {
                    Self::IPropertyMetadataStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateWithFactory)(
                            windows_core::Interface::as_raw(this),
                            createdefaultvaluecallback.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn CreateWithFactoryAndCallback<P0, P1>(
                    createdefaultvaluecallback: P0,
                    propertychangedcallback: P1,
                ) -> windows_core::Result<Self>
                where
                    P0: windows_core::Param<CreateDefaultValueCallback>,
                    P1: windows_core::Param<PropertyChangedCallback>,
                {
                    Self::IPropertyMetadataStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateWithFactoryAndCallback)(
                            windows_core::Interface::as_raw(this),
                            createdefaultvaluecallback.param().abi(),
                            propertychangedcallback.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                fn IPropertyMetadataFactory<
                    R,
                    F: FnOnce(&IPropertyMetadataFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        PropertyMetadata,
                        IPropertyMetadataFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IPropertyMetadataStatics<
                    R,
                    F: FnOnce(&IPropertyMetadataStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        PropertyMetadata,
                        IPropertyMetadataStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for PropertyMetadata {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IPropertyMetadata>();
            }
            unsafe impl windows_core::Interface for PropertyMetadata {
                type Vtable = <IPropertyMetadata as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPropertyMetadata as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PropertyMetadata {
                const NAME: &'static str = "Microsoft.UI.Xaml.PropertyMetadata";
            }
            unsafe impl Send for PropertyMetadata {}
            unsafe impl Sync for PropertyMetadata {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ResourceDictionary(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                ResourceDictionary,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            windows_core::imp::required_hierarchy!(ResourceDictionary, super::super::super::Windows::Foundation::Collections::IIterable < windows_collections::IKeyValuePair < windows_core::IInspectable, windows_core::IInspectable > >, super::super::super::Windows::Foundation::Collections::IMap < windows_core::IInspectable, windows_core::IInspectable >, DependencyObject);
            impl ResourceDictionary {
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<
                        windows_collections::IKeyValuePair<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        super::super::super::Windows::Foundation::Collections::IIterable<
                            windows_collections::IKeyValuePair<
                                windows_core::IInspectable,
                                windows_core::IInspectable,
                            >,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn Lookup<P0>(
                    &self,
                    key: P0,
                ) -> windows_core::Result<windows_core::IInspectable>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        super::super::super::Windows::Foundation::Collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Lookup)(
                            windows_core::Interface::as_raw(this),
                            key.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
                where
                    P0: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<
                        super::super::super::Windows::Foundation::Collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).HasKey)(
                            windows_core::Interface::as_raw(this),
                            key.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn MergedDictionaries(
                    &self,
                ) -> windows_core::Result<windows_collections::IVector<Self>> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).MergedDictionaries)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn ThemeDictionaries(
                    &self,
                ) -> windows_core::Result<
                    super::super::super::Windows::Foundation::Collections::IMap<
                        windows_core::IInspectable,
                        windows_core::IInspectable,
                    >,
                > {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).ThemeDictionaries)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn new() -> windows_core::Result<Self> {
                    Self::IResourceDictionaryFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    })
                }
                pub fn compose<T>(compose: T) -> windows_core::Result<Self>
                where
                    T: windows_core::Compose,
                {
                    Self::IResourceDictionaryFactory(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::imp::Type::from_abi(result__)
                    })
                }
                fn IResourceDictionaryFactory<
                    R,
                    F: FnOnce(&IResourceDictionaryFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        ResourceDictionary,
                        IResourceDictionaryFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for ResourceDictionary {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IResourceDictionary>();
            }
            unsafe impl windows_core::Interface for ResourceDictionary {
                type Vtable = <IResourceDictionary as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IResourceDictionary as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ResourceDictionary {
                const NAME: &'static str = "Microsoft.UI.Xaml.ResourceDictionary";
            }
            unsafe impl Send for ResourceDictionary {}
            unsafe impl Sync for ResourceDictionary {}
            impl IntoIterator for ResourceDictionary {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl IntoIterator for &ResourceDictionary {
                type Item = windows_collections::IKeyValuePair<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct Thickness {
                pub Left: f64,
                pub Top: f64,
                pub Right: f64,
                pub Bottom: f64,
            }
            impl windows_core::imp::TypeKind for Thickness {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for Thickness {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"struct(Microsoft.UI.Xaml.Thickness;f8;f8;f8;f8)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Thickness");
            }
            pub mod Controls {
                pub mod Primitives {
                    #[repr(transparent)]
                    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                    pub struct ComponentResourceLocation(pub i32);
                    impl ComponentResourceLocation {
                        pub const Application: Self = Self(0);
                        pub const Nested: Self = Self(1);
                    }
                    impl windows_core::imp::TypeKind for ComponentResourceLocation {
                        type TypeKind = windows_core::imp::CopyType;
                    }
                    impl windows_core::RuntimeType for ComponentResourceLocation {
                        const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"enum(Microsoft.UI.Xaml.Controls.Primitives.ComponentResourceLocation;i4)") ;
                        const NAME: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::from_slice(
                                b"Microsoft.UI.Xaml.Controls.Primitives.ComponentResourceLocation",
                            );
                    }
                }
            }
            pub mod Media {
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Brush(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    Brush,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                windows_core::imp::required_hierarchy!(Brush, super::DependencyObject);
                impl Brush {
                    pub fn Opacity(&self) -> windows_core::Result<f64> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Opacity)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetOpacity)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IBrushFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
                    where
                        T: windows_core::Compose,
                    {
                        Self::IBrushFactory(|this| unsafe {
                            let (derived__, base__) = windows_core::Compose::compose(compose);
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(&derived__),
                                base__ as *mut _ as _,
                                &mut result__,
                            )
                            .ok()?;
                            let _ = &derived__;
                            windows_core::imp::Type::from_abi(result__)
                        })
                    }
                    pub fn OpacityProperty() -> windows_core::Result<super::DependencyProperty> {
                        Self::IBrushStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).OpacityProperty)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    pub fn TransformProperty() -> windows_core::Result<super::DependencyProperty> {
                        Self::IBrushStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).TransformProperty)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    pub fn RelativeTransformProperty()
                    -> windows_core::Result<super::DependencyProperty> {
                        Self::IBrushStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).RelativeTransformProperty)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    fn IBrushFactory<R, F: FnOnce(&IBrushFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Brush, IBrushFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IBrushStatics<R, F: FnOnce(&IBrushStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Brush, IBrushStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Brush {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IBrush>();
                }
                unsafe impl windows_core::Interface for Brush {
                    type Vtable = <IBrush as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBrush as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Brush {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Brush";
                }
                unsafe impl Send for Brush {}
                unsafe impl Sync for Brush {}
                windows_core::imp::define_interface!(
                    IBrush,
                    IBrush_Vtbl,
                    0x2de3cb83_1329_5679_88f8_c822bc5442cb
                );
                impl windows_core::RuntimeType for IBrush {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Media.IBrush",
                        );
                }
                #[repr(C)]
                pub struct IBrush_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Opacity: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut f64,
                    )
                        -> windows_core::HRESULT,
                    pub SetOpacity: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        f64,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBrushFactory,
                    IBrushFactory_Vtbl,
                    0xb5258717_6c49_5ba5_87fd_35df382647a5
                );
                impl windows_core::RuntimeType for IBrushFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Media.IBrushFactory",
                        );
                }
                #[repr(C)]
                pub struct IBrushFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBrushStatics,
                    IBrushStatics_Vtbl,
                    0x5b854f50_f818_5f01_91b0_28132d3f5957
                );
                impl windows_core::RuntimeType for IBrushStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Media.IBrushStatics",
                        );
                }
                #[repr(C)]
                pub struct IBrushStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OpacityProperty: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub TransformProperty: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                    pub RelativeTransformProperty:
                        unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISolidColorBrush,
                    ISolidColorBrush_Vtbl,
                    0xb3865c31_37c8_55c1_8a72_d41c67642e2a
                );
                impl windows_core::RuntimeType for ISolidColorBrush {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Media.ISolidColorBrush",
                        );
                }
                #[repr(C)]
                pub struct ISolidColorBrush_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Color: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::super::super::super::Windows::UI::Color,
                    )
                        -> windows_core::HRESULT,
                    pub SetColor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::super::super::Windows::UI::Color,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISolidColorBrushFactory,
                    ISolidColorBrushFactory_Vtbl,
                    0x7b559384_4daa_54f4_91ef_33a23fd816ca
                );
                impl windows_core::RuntimeType for ISolidColorBrushFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Media.ISolidColorBrushFactory",
                        );
                }
                #[repr(C)]
                pub struct ISolidColorBrushFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstanceWithColor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::super::super::Windows::UI::Color,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISolidColorBrushStatics,
                    ISolidColorBrushStatics_Vtbl,
                    0x6bc16da0_c4e6_59b8_995b_b31e48424c07
                );
                impl windows_core::RuntimeType for ISolidColorBrushStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Microsoft.UI.Xaml.Media.ISolidColorBrushStatics",
                        );
                }
                #[repr(C)]
                pub struct ISolidColorBrushStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub ColorProperty: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct SolidColorBrush(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    SolidColorBrush,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                windows_core::imp::required_hierarchy!(
                    SolidColorBrush,
                    Brush,
                    super::DependencyObject
                );
                impl SolidColorBrush {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<
                        R,
                        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            SolidColorBrush,
                            windows_core::imp::IGenericFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn Opacity(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<IBrush>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Opacity)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IBrush>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetOpacity)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn Color(
                        &self,
                    ) -> windows_core::Result<super::super::super::super::Windows::UI::Color>
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Color)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetColor(
                        &self,
                        value: super::super::super::super::Windows::UI::Color,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetColor)(
                                windows_core::Interface::as_raw(self),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn CreateInstanceWithColor(
                        color: super::super::super::super::Windows::UI::Color,
                    ) -> windows_core::Result<Self> {
                        Self::ISolidColorBrushFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstanceWithColor)(
                                windows_core::Interface::as_raw(this),
                                color,
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    pub fn ColorProperty() -> windows_core::Result<super::DependencyProperty> {
                        Self::ISolidColorBrushStatics(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).ColorProperty)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::imp::Type::from_abi(result__))
                        })
                    }
                    fn ISolidColorBrushFactory<
                        R,
                        F: FnOnce(&ISolidColorBrushFactory) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            SolidColorBrush,
                            ISolidColorBrushFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn ISolidColorBrushStatics<
                        R,
                        F: FnOnce(&ISolidColorBrushStatics) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            SolidColorBrush,
                            ISolidColorBrushStatics,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for SolidColorBrush {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, ISolidColorBrush>();
                }
                unsafe impl windows_core::Interface for SolidColorBrush {
                    type Vtable = <ISolidColorBrush as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID =
                        <ISolidColorBrush as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for SolidColorBrush {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SolidColorBrush";
                }
                unsafe impl Send for SolidColorBrush {}
                unsafe impl Sync for SolidColorBrush {}
            }
        }
    }
}

#[allow(warnings)]
#[allow(nonstandard_style)]
#[allow(non_snake_case)]
pub mod Windows {
    pub mod Foundation {
        windows_core::imp::define_interface!(
            IPropertyValue,
            IPropertyValue_Vtbl,
            0x4bd682dd_7554_40e9_9a9b_82654ede7e62
        );
        impl windows_core::RuntimeType for IPropertyValue {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IPropertyValue");
        }
        windows_core::imp::interface_hierarchy!(
            IPropertyValue,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl windows_core::RuntimeName for IPropertyValue {
            const NAME: &'static str = "Windows.Foundation.IPropertyValue";
        }
        #[repr(C)]
        pub struct IPropertyValue_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        windows_core::imp::define_interface!(
            IPropertyValueStatics,
            IPropertyValueStatics_Vtbl,
            0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858
        );
        impl windows_core::RuntimeType for IPropertyValueStatics {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Foundation.IPropertyValueStatics",
            );
        }
        #[repr(C)]
        pub struct IPropertyValueStatics_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateEmpty: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt8: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u8,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInt16: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i16,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt16: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u16,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInt32: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt32: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInt64: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                i64,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt64: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u64,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateSingle: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                f32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateDouble: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                f64,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateChar16: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u16,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateBoolean: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                bool,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInspectable: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateGuid: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                windows_core::GUID,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateDateTime: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                windows_time::DateTime,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateTimeSpan: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                windows_time::TimeSpan,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreatePoint: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                Point,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateSize: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                Size,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateRect: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                Rect,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt8Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const u8,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInt16Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const i16,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt16Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const u16,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInt32Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const i32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt32Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const u32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInt64Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const i64,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateUInt64Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const u64,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateSingleArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const f32,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateDoubleArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const f64,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateChar16Array: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const u16,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateBooleanArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const bool,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateStringArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_core::HSTRING,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInspectableArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_core::IInspectable,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub CreateGuidArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_core::GUID,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateDateTimeArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_time::DateTime,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub CreateTimeSpanArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_time::TimeSpan,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
            pub CreatePointArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const Point,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateSizeArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const Size,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateRectArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const Rect,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct IReference<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
        where
            T: windows_core::RuntimeType + 'static;
        impl<T: windows_core::RuntimeType + 'static>
            windows_core::imp::CanInto<windows_core::IUnknown> for IReference<T>
        {
        }
        impl<T: windows_core::RuntimeType + 'static>
            windows_core::imp::CanInto<windows_core::IInspectable> for IReference<T>
        {
        }
        unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IReference<T> {
            type Vtable = IReference_Vtbl<T>;
            const IID: windows_core::GUID =
                windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
        }
        impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IReference<T> {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                .push_slice(b"pinterface({61c17706-2d65-11e0-9ae8-d48564015472}")
                .push_slice(b";")
                .push_other(T::SIGNATURE)
                .push_slice(b")");
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                .push_slice(b"Windows.Foundation.IReference`1<")
                .push_other(T::NAME)
                .push_slice(b">");
        }
        impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IPropertyValue>
            for IReference<T>
        {
            const QUERY: bool = true;
        }
        impl<T: windows_core::RuntimeType + 'static> IReference<T> {
            pub fn Value(&self) -> windows_core::Result<T> {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(self).Value)(
                        windows_core::Interface::as_raw(self),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                }
            }
        }
        impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IReference<T> {
            const NAME: &'static str = "Windows.Foundation.IReference";
            const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
                <Self as windows_core::RuntimeType>::NAME;
        }
        #[repr(C)]
        pub struct IReference_Vtbl<T>
        where
            T: windows_core::RuntimeType + 'static,
        {
            pub base__: windows_core::IInspectable_Vtbl,
            pub Value: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut windows_core::imp::AbiType<T>,
            ) -> windows_core::HRESULT,
            T: core::marker::PhantomData<T>,
        }
        windows_core::imp::define_interface!(
            IUriEscapeStatics,
            IUriEscapeStatics_Vtbl,
            0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1
        );
        impl windows_core::RuntimeType for IUriEscapeStatics {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriEscapeStatics");
        }
        #[repr(C)]
        pub struct IUriEscapeStatics_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub UnescapeComponent: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub EscapeComponent: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IUriRuntimeClass,
            IUriRuntimeClass_Vtbl,
            0x9e365e57_48b2_4160_956f_c7385120bbfc
        );
        impl windows_core::RuntimeType for IUriRuntimeClass {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClass");
        }
        #[repr(C)]
        pub struct IUriRuntimeClass_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        windows_core::imp::define_interface!(
            IUriRuntimeClassFactory,
            IUriRuntimeClassFactory_Vtbl,
            0x44a9796f_723e_4fdf_a218_033e75b0c084
        );
        impl windows_core::RuntimeType for IUriRuntimeClassFactory {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
            const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                b"Windows.Foundation.IUriRuntimeClassFactory",
            );
        }
        #[repr(C)]
        pub struct IUriRuntimeClassFactory_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateWithRelativeUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            )
                -> windows_core::HRESULT,
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Point {
            pub X: f32,
            pub Y: f32,
        }
        impl windows_core::imp::TypeKind for Point {
            type TypeKind = windows_core::imp::CopyType;
        }
        impl windows_core::RuntimeType for Point {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Windows.Foundation.Point;f4;f4)",
                );
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.Point");
        }
        pub struct PropertyValue;
        impl PropertyValue {
            pub fn CreateEmpty() -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateEmpty)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt8(value: u8) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt8)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInt16(value: i16) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInt16)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt16(value: u16) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt16)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInt32(value: i32) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInt32)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt32(value: u32) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt32)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInt64(value: i64) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInt64)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt64(value: u64) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt64)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateSingle(value: f32) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateSingle)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateDouble(value: f64) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateDouble)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateChar16(value: u16) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateChar16)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateBoolean(value: bool) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateBoolean)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateString(
                value: &windows_core::HSTRING,
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateString)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(value),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInspectable<P0>(
                value: P0,
            ) -> windows_core::Result<windows_core::IInspectable>
            where
                P0: windows_core::Param<windows_core::IInspectable>,
            {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInspectable)(
                        windows_core::Interface::as_raw(this),
                        value.param().abi(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateGuid(
                value: windows_core::GUID,
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateGuid)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateDateTime(
                value: windows_time::DateTime,
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateDateTime)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateTimeSpan(
                value: windows_time::TimeSpan,
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateTimeSpan)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreatePoint(value: Point) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreatePoint)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateSize(value: Size) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateSize)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateRect(value: Rect) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateRect)(
                        windows_core::Interface::as_raw(this),
                        value,
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt8Array(
                value: &[u8],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt8Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInt16Array(
                value: &[i16],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInt16Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt16Array(
                value: &[u16],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt16Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInt32Array(
                value: &[i32],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInt32Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt32Array(
                value: &[u32],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt32Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInt64Array(
                value: &[i64],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInt64Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateUInt64Array(
                value: &[u64],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUInt64Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateSingleArray(
                value: &[f32],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateSingleArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateDoubleArray(
                value: &[f64],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateDoubleArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateChar16Array(
                value: &[u16],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateChar16Array)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateBooleanArray(
                value: &[bool],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateBooleanArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateStringArray(
                value: &[windows_core::HSTRING],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateStringArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        core::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateInspectableArray(
                value: &[Option<windows_core::IInspectable>],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateInspectableArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        core::mem::transmute(value.as_ptr()),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateGuidArray(
                value: &[windows_core::GUID],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateGuidArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateDateTimeArray(
                value: &[windows_time::DateTime],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateDateTimeArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateTimeSpanArray(
                value: &[windows_time::TimeSpan],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateTimeSpanArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreatePointArray(
                value: &[Point],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreatePointArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateSizeArray(
                value: &[Size],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateSizeArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateRectArray(
                value: &[Rect],
            ) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateRectArray)(
                        windows_core::Interface::as_raw(this),
                        value.len().try_into().unwrap(),
                        value.as_ptr(),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            fn IPropertyValueStatics<
                R,
                F: FnOnce(&IPropertyValueStatics) -> windows_core::Result<R>,
            >(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<
                    PropertyValue,
                    IPropertyValueStatics,
                > = windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeName for PropertyValue {
            const NAME: &'static str = "Windows.Foundation.PropertyValue";
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Rect {
            pub X: f32,
            pub Y: f32,
            pub Width: f32,
            pub Height: f32,
        }
        impl windows_core::imp::TypeKind for Rect {
            type TypeKind = windows_core::imp::CopyType;
        }
        impl windows_core::RuntimeType for Rect {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)",
                );
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.Rect");
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Size {
            pub Width: f32,
            pub Height: f32,
        }
        impl windows_core::imp::TypeKind for Size {
            type TypeKind = windows_core::imp::CopyType;
        }
        impl windows_core::RuntimeType for Size {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Windows.Foundation.Size;f4;f4)",
                );
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.Size");
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Uri(windows_core::IUnknown);
        windows_core::imp::interface_hierarchy!(
            Uri,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        impl Uri {
            pub fn UnescapeComponent(
                tounescape: &windows_core::HSTRING,
            ) -> windows_core::Result<windows_core::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).UnescapeComponent)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(tounescape),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                })
            }
            pub fn EscapeComponent(
                toescape: &windows_core::HSTRING,
            ) -> windows_core::Result<windows_core::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).EscapeComponent)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(toescape),
                        &mut result__,
                    )
                    .map(|| core::mem::transmute(result__))
                })
            }
            pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Self> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUri)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(uri),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            pub fn CreateWithRelativeUri(
                baseuri: &windows_core::HSTRING,
                relativeuri: &windows_core::HSTRING,
            ) -> windows_core::Result<Self> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateWithRelativeUri)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(baseuri),
                        core::mem::transmute_copy(relativeuri),
                        &mut result__,
                    )
                    .and_then(|| windows_core::imp::Type::from_abi(result__))
                })
            }
            fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
            fn IUriRuntimeClassFactory<
                R,
                F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>,
            >(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeType for Uri {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
        }
        unsafe impl windows_core::Interface for Uri {
            type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
            const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
        }
        impl windows_core::RuntimeName for Uri {
            const NAME: &'static str = "Windows.Foundation.Uri";
        }
        unsafe impl Send for Uri {}
        unsafe impl Sync for Uri {}
        pub mod Collections {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
            where
                T: windows_core::RuntimeType + 'static;
            impl<T: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IUnknown> for IIterable<T>
            {
            }
            impl<T: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IInspectable> for IIterable<T>
            {
            }
            unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
                type Vtable = IIterable_Vtbl<T>;
                const IID: windows_core::GUID = windows_core::GUID::from_signature(
                    <Self as windows_core::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::new()
                        .push_slice(b"pinterface({faa585ea-6214-4217-afda-7f46de5869b3}")
                        .push_slice(b";")
                        .push_other(T::SIGNATURE)
                        .push_slice(b")");
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                    .push_slice(b"Windows.Foundation.Collections.IIterable`1<")
                    .push_other(T::NAME)
                    .push_slice(b">");
            }
            impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
                pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<T>> {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).First)(
                            windows_core::Interface::as_raw(self),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterable<T> {
                const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
                const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
                    <Self as windows_core::RuntimeType>::NAME;
            }
            pub trait IIterable_Impl<T>: windows_core::IUnknownImpl
            where
                T: windows_core::RuntimeType + 'static,
            {
                fn First(&self) -> windows_core::Result<windows_collections::IIterator<T>>;
            }
            impl<T: windows_core::RuntimeType + 'static> IIterable_Vtbl<T> {
                pub const fn new<Identity: IIterable_Impl<T>, const OFFSET: isize>() -> Self {
                    unsafe extern "system" fn First<
                        T: windows_core::RuntimeType + 'static,
                        Identity: IIterable_Impl<T>,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        result__: *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            match IIterable_Impl::First(this) {
                                Ok(ok__) => {
                                    result__.write(core::mem::transmute_copy(&ok__));
                                    core::mem::forget(ok__);
                                    windows_core::HRESULT(0)
                                }
                                Err(err) => err.into(),
                            }
                        }
                    }
                    Self {
                        base__: windows_core::IInspectable_Vtbl::new::<
                            Identity,
                            IIterable<T>,
                            OFFSET,
                        >(),
                        First: First::<T, Identity, OFFSET>,
                        T: core::marker::PhantomData::<T>,
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IIterable<T> as windows_core::Interface>::IID
                }
            }
            #[repr(C)]
            pub struct IIterable_Vtbl<T>
            where
                T: windows_core::RuntimeType + 'static,
            {
                pub base__: windows_core::IInspectable_Vtbl,
                pub First: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                T: core::marker::PhantomData<T>,
            }
            impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
                type Item = T;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
                type Item = T;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct IMap<K, V>(
                windows_core::IUnknown,
                core::marker::PhantomData<K>,
                core::marker::PhantomData<V>,
            )
            where
                K: windows_core::RuntimeType + 'static,
                V: windows_core::RuntimeType + 'static;
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IUnknown> for IMap<K, V>
            {
            }
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<windows_core::IInspectable> for IMap<K, V>
            {
            }
            unsafe impl<
                K: windows_core::RuntimeType + 'static,
                V: windows_core::RuntimeType + 'static,
            > windows_core::Interface for IMap<K, V>
            {
                type Vtable = IMap_Vtbl<K, V>;
                const IID: windows_core::GUID = windows_core::GUID::from_signature(
                    <Self as windows_core::RuntimeType>::SIGNATURE,
                );
            }
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
                windows_core::RuntimeType for IMap<K, V>
            {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::new()
                        .push_slice(b"pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1}")
                        .push_slice(b";")
                        .push_other(K::SIGNATURE)
                        .push_slice(b";")
                        .push_other(V::SIGNATURE)
                        .push_slice(b")");
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                    .push_slice(b"Windows.Foundation.Collections.IMap`2<")
                    .push_other(K::NAME)
                    .push_slice(b", ")
                    .push_other(V::NAME)
                    .push_slice(b">");
            }
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
                windows_core::imp::CanInto<IIterable<windows_collections::IKeyValuePair<K, V>>>
                for IMap<K, V>
            {
                const QUERY: bool = true;
            }
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMap<K, V> {
                pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
                where
                    P0: windows_core::Param<K>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).Lookup)(
                            windows_core::Interface::as_raw(self),
                            key.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
                pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
                where
                    P0: windows_core::Param<K>,
                {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(self).HasKey)(
                            windows_core::Interface::as_raw(self),
                            key.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<windows_collections::IKeyValuePair<K, V>>,
                > {
                    let this = &windows_core::Interface::cast::<
                        IIterable<windows_collections::IKeyValuePair<K, V>>,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::imp::Type::from_abi(result__))
                    }
                }
            }
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
                IntoIterator for IMap<K, V>
            {
                type Item = windows_collections::IKeyValuePair<K, V>;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
                IntoIterator for &IMap<K, V>
            {
                type Item = windows_collections::IKeyValuePair<K, V>;
                type IntoIter = windows_collections::BufferedIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    windows_collections::BufferedIterator::new(self.First().unwrap())
                }
            }
            impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
                windows_core::RuntimeName for IMap<K, V>
            {
                const NAME: &'static str = "Windows.Foundation.Collections.IMap";
                const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
                    <Self as windows_core::RuntimeType>::NAME;
            }
            #[repr(C)]
            pub struct IMap_Vtbl<K, V>
            where
                K: windows_core::RuntimeType + 'static,
                V: windows_core::RuntimeType + 'static,
            {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Lookup: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows_core::imp::AbiType<K>,
                    *mut windows_core::imp::AbiType<V>,
                ) -> windows_core::HRESULT,
                Size: usize,
                pub HasKey: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows_core::imp::AbiType<K>,
                    *mut bool,
                ) -> windows_core::HRESULT,
                K: core::marker::PhantomData<K>,
                V: core::marker::PhantomData<V>,
            }
        }
    }
    pub mod UI {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct Color {
            pub A: u8,
            pub R: u8,
            pub G: u8,
            pub B: u8,
        }
        impl windows_core::imp::TypeKind for Color {
            type TypeKind = windows_core::imp::CopyType;
        }
        impl windows_core::RuntimeType for Color {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Color");
        }
        pub mod Xaml {
            pub mod Interop {
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct TypeKind(pub i32);
                impl TypeKind {
                    pub const Primitive: Self = Self(0);
                    pub const Metadata: Self = Self(1);
                    pub const Custom: Self = Self(2);
                }
                impl windows_core::imp::TypeKind for TypeKind {
                    type TypeKind = windows_core::imp::CopyType;
                }
                impl windows_core::RuntimeType for TypeKind {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)",
                        );
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Windows.UI.Xaml.Interop.TypeKind",
                        );
                }
                #[repr(C)]
                #[derive(Clone, Debug, Default, Eq, PartialEq)]
                pub struct TypeName {
                    pub Name: windows_core::HSTRING,
                    pub Kind: TypeKind,
                }
                impl windows_core::imp::TypeKind for TypeName {
                    type TypeKind = windows_core::imp::CloneType;
                }
                impl windows_core::RuntimeType for TypeName {
                    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))") ;
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(
                            b"Windows.UI.Xaml.Interop.TypeName",
                        );
                }
            }
        }
    }
}

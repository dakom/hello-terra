!function(){"use strict";function n(n){const e=process.env[n];return null==e?"":e}let e;!function(){const n={REMOTE_TARGET:"release"};try{if(process)return process.env=Object.assign({},process.env),void Object.assign(process.env,n)}catch(n){}globalThis.process={env:n}}();const t=new Array(32).fill(void 0);function r(n){return t[n]}t.push(void 0,null,!0,!1);let _=t.length;function c(n){_===t.length&&t.push(t.length+1);const e=_;return _=t[e],t[e]=n,e}function o(n){const e=r(n);return function(n){n<36||(t[n]=_,_=n)}(n),e}function u(n){return null==n}let b=null;let i=null;function a(){return null!==i&&i.buffer===e.memory.buffer||(i=new Int32Array(e.memory.buffer)),i}let f=0,g=null;function w(){return null!==g&&g.buffer===e.memory.buffer||(g=new Uint8Array(e.memory.buffer)),g}let s=new TextEncoder("utf-8");const l="function"==typeof s.encodeInto?function(n,e){return s.encodeInto(n,e)}:function(n,e){const t=s.encode(n);return e.set(t),{read:n.length,written:t.length}};function d(n,e,t){if(void 0===t){const t=s.encode(n),r=e(t.length);return w().subarray(r,r+t.length).set(t),f=t.length,r}let r=n.length,_=e(r);const c=w();let o=0;for(;o<r;o++){const e=n.charCodeAt(o);if(e>127)break;c[_+o]=e}if(o!==r){0!==o&&(n=n.slice(o)),_=t(_,r,r=o+3*n.length);const e=w().subarray(_+o,_+r);o+=l(n,e).written}return f=o,_}let m=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});function v(n,e){return m.decode(w().subarray(n,n+e))}function y(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=y(n[0]));for(let r=1;r<e;r++)t+=", "+y(n[r]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if(r=t[1],"Object"==r)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}function p(n,t,r,_){const c={a:n,b:t,cnt:1,dtor:r},o=(...n)=>{c.cnt++;const t=c.a;c.a=0;try{return _(t,c.b,...n)}finally{0==--c.cnt?e.__wbindgen_export_2.get(c.dtor)(t,c.b):c.a=t}};return o.original=c,o}m.decode();let h=32;function A(n,r,_){try{e._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8f192a5d42883fb0(n,r,function(n){if(1==h)throw new Error("out of js stack");return t[--h]=n,h}(_))}finally{t[h++]=void 0}}function E(n,t,r){e._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbde36d197208e0f9(n,t,c(r))}function R(n,e){return 0===n?r(e):v(n,e)}function S(n,t){try{return n.apply(this,t)}catch(n){e.__wbindgen_exn_store(c(n))}}(async function t(_){void 0===_&&(_=new URL("index_bg.wasm",document.currentScript&&document.currentScript.src||new URL("app.js",document.baseURI).href));const i={wbg:{}};i.wbg.__wbindgen_object_clone_ref=function(n){return c(r(n))},i.wbg.__wbg_value_bc4bb925ad58795b=function(n,t){var _=d(r(t).value,e.__wbindgen_malloc,e.__wbindgen_realloc),c=f;a()[n/4+1]=c,a()[n/4+0]=_},i.wbg.__wbindgen_object_drop_ref=function(n){o(n)},i.wbg.__wbg_abort_98e2a31e5ecfeee6=function(n){r(n).abort()},i.wbg.__wbindgen_cb_drop=function(n){const e=o(n).original;if(1==e.cnt--)return e.a=0,!0;return!1},i.wbg.__wbg_removeEventListener_24d5a7c12c3f3c39=function(){return S((function(n,e,t,_,c){var o=R(e,t);r(n).removeEventListener(o,r(_),0!==c)}),arguments)},i.wbg.__wbg_instanceof_HtmlElement_df66c8b4a687aa43=function(n){return r(n)instanceof HTMLElement},i.wbg.__wbg_classList_b666640fdfbcc8ab=function(n){return c(r(n).classList)},i.wbg.__wbg_add_f36d97e1d70d27b0=function(){return S((function(n,e,t){var _=R(e,t);r(n).add(_)}),arguments)},i.wbg.__wbg_appendChild_d318db34c4559916=function(){return S((function(n,e){return c(r(n).appendChild(r(e)))}),arguments)},i.wbg.__wbg_style_c88e323890d3a091=function(n){return c(r(n).style)},i.wbg.__wbg_addevent_bcce2546b34a5dea=function(n,e,t,_,c,o){var u,b,i,a,f,g=R(e,t);u=r(n),b=g,i=0!==_,a=0!==c,f=r(o),u.addEventListener(b,f,{capture:i,passive:a,once:!1})},i.wbg.__wbg_instanceof_HtmlInputElement_8cafe5f30dfdb6bc=function(n){return r(n)instanceof HTMLInputElement},i.wbg.__wbg_setAttribute_1b533bf07966de55=function(){return S((function(n,e,t,_,c){var o=R(e,t),u=R(_,c);r(n).setAttribute(o,u)}),arguments)},i.wbg.__wbg_data_9e55e7d79ab13ef1=function(n){return c(r(n).data)},i.wbg.__wbg_isArray_eb7ad55f2da67dde=function(n){return Array.isArray(r(n))},i.wbg.__wbg_values_364ae56c608e6824=function(n){return c(r(n).values())},i.wbg.__wbg_next_7720502039b96d00=function(){return S((function(n){return c(r(n).next())}),arguments)},i.wbg.__wbg_done_b06cf0578e89ff68=function(n){return r(n).done},i.wbg.__wbg_value_e74a542443d92451=function(n){return c(r(n).value)},i.wbg.__wbindgen_number_get=function(n,t){const _=r(t);var c="number"==typeof _?_:void 0;(null!==b&&b.buffer===e.memory.buffer||(b=new Float64Array(e.memory.buffer)),b)[n/8+1]=u(c)?0:c,a()[n/4+0]=!u(c)},i.wbg.__wbg_isSafeInteger_0dfc6d38b7184f06=function(n){return Number.isSafeInteger(r(n))},i.wbg.__wbindgen_string_get=function(n,t){const _=r(t);var c="string"==typeof _?_:void 0,o=u(c)?0:d(c,e.__wbindgen_malloc,e.__wbindgen_realloc),b=f;a()[n/4+1]=b,a()[n/4+0]=o},i.wbg.__wbindgen_is_string=function(n){return"string"==typeof r(n)},i.wbg.__wbg_length_42e02f5a04d67464=function(n){return r(n).length},i.wbg.__wbg_get_67189fe0b323d288=function(n,e){return c(r(n)[e>>>0])},i.wbg.__wbindgen_is_null=function(n){return null===r(n)},i.wbg.__wbindgen_is_undefined=function(n){return void 0===r(n)},i.wbg.__wbindgen_boolean_get=function(n){const e=r(n);return"boolean"==typeof e?e?1:0:2},i.wbg.__wbindgen_is_object=function(n){const e=r(n);return"object"==typeof e&&null!==e},i.wbg.__wbg_iterator_4fc4ce93e6b92958=function(){return c(Symbol.iterator)},i.wbg.__wbg_has_1275b5eec3dc7a7a=function(){return S((function(n,e){return Reflect.has(r(n),r(e))}),arguments)},i.wbg.__wbg_entries_aadf9c3f38203a12=function(n){return c(Object.entries(r(n)))},i.wbg.__wbg_get_2d1407dba3452350=function(n,e){return c(r(n)[o(e)])},i.wbg.__wbg_new_342a24ca698edd87=function(n,e){var t=R(n,e);return c(new Error(t))},i.wbg.__wbg_removeevent_48987e6b03331150=function(n,e,t,_,c){var o,u,b,i,a=R(e,t);o=r(n),u=a,b=0!==_,i=r(c),o.removeEventListener(u,i,b)},i.wbg.__wbg_new_0b83d3df67ecb33e=function(){return c(new Object)},i.wbg.__wbindgen_string_new=function(n,e){return c(v(n,e))},i.wbg.__wbg_set_f1a4ac8f3a605b11=function(n,e,t){r(n)[o(e)]=o(t)},i.wbg.__wbg_new_949bbc1147195c4e=function(){return c(new Array)},i.wbg.__wbg_push_284486ca27c6aa8b=function(n,e){return r(n).push(r(e))},i.wbg.__wbindgen_number_new=function(n){return c(n)},i.wbg.__wbg_setproperty_8d813455ae0ac1e6=function(n,e,t,_){var c,o,u,b=R(e,t);c=r(n),o=b,u=r(_),c[o]=u},i.wbg.__wbg_addEventListener_09e11fbf8b4b719b=function(){return S((function(n,e,t,_,c){var o=R(e,t);r(n).addEventListener(o,r(_),r(c))}),arguments)},i.wbg.__wbg_contentWindow_c021bb036e982fa2=function(n){var e=r(n).contentWindow;return u(e)?0:c(e)},i.wbg.__wbg_postMessage_7d70647a5e45cb0f=function(){return S((function(n,e,t,_){var c=R(t,_);r(n).postMessage(r(e),c)}),arguments)},i.wbg.__wbg_localStorage_6775414303ab5085=function(){return S((function(n){var e=r(n).localStorage;return u(e)?0:c(e)}),arguments)},i.wbg.__wbg_set_3a236a97145dc780=function(){return S((function(n,e,t,_,c){var o=R(e,t),u=R(_,c);r(n)[o]=u}),arguments)},i.wbg.__wbg_get_f0092ad67dc97639=function(){return S((function(n,t,_,c){var o=R(_,c),b=r(t)[o],i=u(b)?0:d(b,e.__wbindgen_malloc,e.__wbindgen_realloc),g=f;a()[n/4+1]=g,a()[n/4+0]=i}),arguments)},i.wbg.__wbg_processenvvar_07d75200f6f49ccd=function(){return S((function(t,r,_){var c=d(n(R(r,_)),e.__wbindgen_malloc,e.__wbindgen_realloc),o=f;a()[t/4+1]=o,a()[t/4+0]=c}),arguments)},i.wbg.__wbg_remove_89670e56a41482a8=function(){return S((function(n,e,t){var _=R(e,t);r(n).remove(_)}),arguments)},i.wbg.__wbg_instanceof_HtmlSelectElement_27fb687660e6b5ba=function(n){return r(n)instanceof HTMLSelectElement},i.wbg.__wbg_setdata_903bd12d7af0d5b5=function(n,e,t){var _=R(e,t);r(n).data=_},i.wbg.__wbg_insertBefore_5b314357408fbec1=function(){return S((function(n,e,t){return c(r(n).insertBefore(r(e),r(t)))}),arguments)},i.wbg.__wbg_document_1c64944725c0d81d=function(n){var e=r(n).document;return u(e)?0:c(e)},i.wbg.__wbg_body_78ae4fd43b446013=function(n){var e=r(n).body;return u(e)?0:c(e)},i.wbg.__wbg_instanceof_HtmlIFrameElement_efaa0c47a7fa82f4=function(n){return r(n)instanceof HTMLIFrameElement},i.wbg.__wbg_newwithstr_226291f201e32f74=function(){return S((function(n,e){var t=R(n,e);return c(new Request(t))}),arguments)},i.wbg.__wbg_signal_7db5cc97f02e262b=function(n){return c(r(n).signal)},i.wbg.__wbg_new_4cea363b8cc0002c=function(){return S((function(){return c(new AbortController)}),arguments)},i.wbg.__wbg_fetch_e02c1fac507a760a=function(n,e,t){return c(r(n).fetch(r(e),r(t)))},i.wbg.__wbg_arrayBuffer_b8937ed04beb0d36=function(){return S((function(n){return c(r(n).arrayBuffer())}),arguments)},i.wbg.__wbg_new_a7ce447f15ff496f=function(n){return c(new Uint8Array(r(n)))},i.wbg.__wbg_alert_70f063b2a1df2d0f=function(){return S((function(n,e,t){var _=R(e,t);r(n).alert(_)}),arguments)},i.wbg.__wbg_text_8279d34d73e43c68=function(){return S((function(n){return c(r(n).text())}),arguments)},i.wbg.__wbg_String_c8baaa0740def8c6=function(n,t){var _=d(String(r(t)),e.__wbindgen_malloc,e.__wbindgen_realloc),c=f;a()[n/4+1]=c,a()[n/4+0]=_},i.wbg.__wbg_createElement_86c152812a141a62=function(){return S((function(n,e,t){var _=R(e,t);return c(r(n).createElement(_))}),arguments)},i.wbg.__wbg_settype_f777a49b612d94f0=function(n,e,t){var _=R(e,t);r(n).type=_},i.wbg.__wbg_head_d205ec9bd59f31a7=function(n){var e=r(n).head;return u(e)?0:c(e)},i.wbg.__wbg_sheet_6b235d2f91d4d2c1=function(n){var e=r(n).sheet;return u(e)?0:c(e)},i.wbg.__wbg_cssRules_b9ce8ac851304351=function(){return S((function(n){return c(r(n).cssRules)}),arguments)},i.wbg.__wbg_length_1ac7ec4672c36486=function(n){return r(n).length},i.wbg.__wbg_insertRule_e5808a97c0c5ecbe=function(){return S((function(n,e,t,_){var c=R(e,t);return r(n).insertRule(c,_>>>0)}),arguments)},i.wbg.__wbg_get_04c5c697dff89315=function(n,e){var t=r(n)[e>>>0];return u(t)?0:c(t)},i.wbg.__wbg_style_fcef2171d2192afa=function(n){return c(r(n).style)},i.wbg.__wbg_removeProperty_150229ec3a3550ad=function(){return S((function(n,t,_,c){var o=R(_,c),u=d(r(t).removeProperty(o),e.__wbindgen_malloc,e.__wbindgen_realloc),b=f;a()[n/4+1]=b,a()[n/4+0]=u}),arguments)},i.wbg.__wbg_setProperty_389eb1a127ad49a5=function(){return S((function(n,e,t,_,c,o,u){var b=R(e,t),i=R(_,c),a=R(o,u);r(n).setProperty(b,i,a)}),arguments)},i.wbg.__wbg_getPropertyValue_a3980b6b5e7fd8a9=function(){return S((function(n,t,_,c){var o=R(_,c),u=d(r(t).getPropertyValue(o),e.__wbindgen_malloc,e.__wbindgen_realloc),b=f;a()[n/4+1]=b,a()[n/4+0]=u}),arguments)},i.wbg.__wbg_createTextNode_365db3bc3d0523ab=function(n,e,t){var _=R(e,t);return c(r(n).createTextNode(_))},i.wbg.__wbg_createComment_710531dc62f02c0c=function(n,e,t){var _=R(e,t);return c(r(n).createComment(_))},i.wbg.__wbg_removeChild_d3ca7b53e537867e=function(){return S((function(n,e){return c(r(n).removeChild(r(e)))}),arguments)},i.wbg.__wbg_target_cc69dde6c2d9ec90=function(n){var e=r(n).target;return u(e)?0:c(e)},i.wbg.__wbg_value_0627d4b1c27534e6=function(n,t){var _=d(r(t).value,e.__wbindgen_malloc,e.__wbindgen_realloc),c=f;a()[n/4+1]=c,a()[n/4+0]=_},i.wbg.__wbg_instanceof_HtmlTextAreaElement_c2f3b4bd6871d5ad=function(n){return r(n)instanceof HTMLTextAreaElement},i.wbg.__wbg_value_686b2a68422cb88d=function(n,t){var _=d(r(t).value,e.__wbindgen_malloc,e.__wbindgen_realloc),c=f;a()[n/4+1]=c,a()[n/4+0]=_},i.wbg.__wbg_get_4d0f21c2f823742e=function(){return S((function(n,e){return c(Reflect.get(r(n),r(e)))}),arguments)},i.wbg.__wbindgen_is_function=function(n){return"function"==typeof r(n)},i.wbg.__wbg_call_888d259a5fefc347=function(){return S((function(n,e){return c(r(n).call(r(e)))}),arguments)},i.wbg.__wbg_next_c4151d46d5fa7097=function(n){return c(r(n).next)},i.wbg.__wbg_length_1eb8fc608a0d4cdb=function(n){return r(n).length},i.wbg.__wbindgen_memory=function(){return c(e.memory)},i.wbg.__wbg_buffer_397eaa4d72ee94dd=function(n){return c(r(n).buffer)},i.wbg.__wbg_set_969ad0a60e51d320=function(n,e,t){r(n).set(r(e),t>>>0)},i.wbg.__wbg_instanceof_Uint8Array_08a1f3a179095e76=function(n){return r(n)instanceof Uint8Array},i.wbg.__wbg_instanceof_ArrayBuffer_764b6d4119231cb3=function(n){return r(n)instanceof ArrayBuffer},i.wbg.__wbindgen_debug_string=function(n,t){var _=d(y(r(t)),e.__wbindgen_malloc,e.__wbindgen_realloc),c=f;a()[n/4+1]=c,a()[n/4+0]=_},i.wbg.__wbindgen_throw=function(n,e){throw new Error(v(n,e))},i.wbg.__wbg_then_8c2d62e8ae5978f7=function(n,e,t){return c(r(n).then(r(e),r(t)))},i.wbg.__wbg_resolve_d23068002f584f22=function(n){return c(Promise.resolve(r(n)))},i.wbg.__wbg_then_2fcac196782070cc=function(n,e){return c(r(n).then(r(e)))},i.wbg.__wbg_set_82a4e8a85e31ac42=function(){return S((function(n,e,t){return Reflect.set(r(n),r(e),r(t))}),arguments)},i.wbg.__wbg_self_c6fbdfc2918d5e58=function(){return S((function(){return c(self.self)}),arguments)},i.wbg.__wbg_window_baec038b5ab35c54=function(){return S((function(){return c(window.window)}),arguments)},i.wbg.__wbg_globalThis_3f735a5746d41fbd=function(){return S((function(){return c(globalThis.globalThis)}),arguments)},i.wbg.__wbg_global_1bc0b39582740e95=function(){return S((function(){return c(global.global)}),arguments)},i.wbg.__wbg_newnoargs_be86524d73f67598=function(n,e){var t=R(n,e);return c(new Function(t))},i.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec=function(n){return r(n)instanceof Window},i.wbg.__wbindgen_closure_wrapper337=function(n,e,t){return c(p(n,e,31,A))},i.wbg.__wbindgen_closure_wrapper1253=function(n,e,t){return c(p(n,e,31,E))},("string"==typeof _||"function"==typeof Request&&_ instanceof Request||"function"==typeof URL&&_ instanceof URL)&&(_=fetch(_));const{instance:g,module:w}=await async function(n,e){if("function"==typeof Response&&n instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,e)}catch(e){if("application/wasm"==n.headers.get("Content-Type"))throw e;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e)}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}(await _,i);return e=g.exports,t.__wbindgen_wasm_module=w,e.__wbindgen_start(),e})("/hello-terra/assets/app-4ca880f5.wasm").catch(console.error)}();

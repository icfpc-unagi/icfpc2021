(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(n,t,e){"use strict";e.r(t);var r=e(2);!async function(){var n=document.getElementById("_problem_id"),t=document.getElementById("_load_init"),e=document.getElementById("_load_best"),o=document.getElementById("_container"),u=document.getElementById("_pose"),i=document.getElementById("_message"),c=document.getElementById("_morph"),l=document.getElementById("_chokudai"),f={problem:"",pose:""};async function s(t){if(!t.startsWith("#"))return;let e=Object.fromEntries(t.substr(1).split("&").map(n=>n.split("=",2).map(n=>decodeURIComponent(n))));e.problem_id&&(n.value=e.problem_id,e.problem_url=`/static/problems/${e.problem_id}.json`);let r=e.problem_url&&fetch(e.problem_url).then(n=>n.ok&&n.text()),o=e.pose_url&&fetch(e.pose_url).then(n=>n.ok&&n.text());f.problem=r&&await r,f.pose=e.pose||o&&await o||"",d(f.problem,f.pose)}function d(n,t){if(t&&n)o.innerHTML=r.M(n,t),i.textContent=`dislikes: ${r.K(n,t)} ${r.O(n,t)}`,u.value=t,u.style.boxShadow="";else{if(!n)return o.innerHTML="",i.textContent="",u.value="",u.style.boxShadow="",void console.debug(arguments);o.innerHTML=r.N(n),i.textContent="",u.value="",u.style.boxShadow=""}let e=n=>{let t=parseInt(n.target.getAttribute("i")),e=n.ctrlKey?10:1,r=n.ctrlKey?Math.PI/2:Math.PI/12,o=n.ctrlKey?.5:.9;switch(n.key){case"ArrowUp":return a(t,n=>[n[0],n[1]-e],n.shiftKey),!1;case"ArrowDown":return a(t,n=>[n[0],n[1]+e],n.shiftKey),!1;case"ArrowLeft":return a(t,n=>[n[0]-e,n[1]],n.shiftKey),!1;case"ArrowRight":return a(t,n=>[n[0]+e,n[1]],n.shiftKey),!1;case"h":return a(t,(n,t)=>[2*t[0]-n[0],n[1]],!0),!1;case"v":return a(t,(n,t)=>[n[0],2*t[1]-n[1]],!0),!1;case",":return a(t,(n,t)=>[Math.round((n[0]-t[0])*Math.cos(-r)-(n[1]-t[1])*Math.sin(-r)+t[0]),Math.round((n[0]-t[0])*Math.sin(-r)+(n[1]-t[1])*Math.cos(-r)+t[1])],!0),!1;case".":return a(t,(n,t)=>[Math.round((n[0]-t[0])*Math.cos(r)-(n[1]-t[1])*Math.sin(r)+t[0]),Math.round((n[0]-t[0])*Math.sin(r)+(n[1]-t[1])*Math.cos(r)+t[1])],!0),!1;case"-":return a(t,(n,t)=>[Math.round((n[0]-t[0])*o+t[0]),Math.round((n[1]-t[1])*o+t[1])],!0),!1;case"+":return a(t,(n,t)=>[Math.round((n[0]-t[0])/o+t[0]),Math.round((n[1]-t[1])/o+t[1])],!0),!1}};document.querySelectorAll("circle[i]").forEach(n=>n.addEventListener("keydown",e))}function a(n,t,e){if(!f.pose)return;let r=JSON.parse(f.pose);for(let o of e?r.vertices.keys():[n])r.vertices[o]=t(r.vertices[o],r.vertices[n]);f.pose=JSON.stringify(r),d(f.problem,f.pose),document.querySelector(`circle[i="${n}"]`).focus()}addEventListener("hashchange",()=>s(location.href),!1),n.addEventListener("change",n=>{let t="#problem_id="+n.target.value;history.replaceState(null,"",t),s(t)}),await s(location.hash),t.addEventListener("click",()=>{f.pose=f.problem&&JSON.stringify({vertices:JSON.parse(f.problem).figure.vertices}),d(f.problem,f.pose)}),e.addEventListener("click",()=>{let t=n.value;if(!t)return;let e=`#problem_id=${t}&pose_url=%2Fbest_solution%3Fproblem_id%3D${t}`;history.replaceState(null,"",e),s(e)}),u.addEventListener("change",n=>{try{f.pose=JSON.stringify(JSON.parse(n.target.value)),d(f.problem,f.pose),n.target.style.boxShadow=""}catch(t){console.error(t),i.textContent=t.toString(),n.target.style.boxShadow="0 0 2px 2px red"}}),o.addEventListener("click",n=>{let t=1/0,e=null;o.querySelectorAll("circle[tabindex]").forEach(r=>{let o=r.getBoundingClientRect(),u=(o.left+o.right)/2-n.clientX,i=(o.top+o.bottom)/2-n.clientY,c=u*u+i*i;c<t&&(t=c,e=r)}),e&&e.focus()}),c.addEventListener("click",(async function(){if(f.problem&&f.pose&&!this.disabled){this.disabled=!0;for(let n=0;n<10&&this.disabled;n++)f.pose=r.L(f.problem,f.pose,1e3),d(f.problem,f.pose),await new Promise(n=>setTimeout(n,10));this.disabled=!1}})),l.addEventListener("click",(async function(){if(f.problem&&f.pose&&!this.disabled){this.disabled=!0;try{f.pose=r.J(f.problem,f.pose,1,!0,!0),d(f.problem,f.pose),this.disabled=!1}catch(n){i.textContent=n.toString(),console.error(n)}}}))}()},function(n,t,e){"use strict";(function(n,r){e.d(t,"N",(function(){return g})),e.d(t,"M",(function(){return x})),e.d(t,"K",(function(){return E})),e.d(t,"O",(function(){return _})),e.d(t,"L",(function(){return M})),e.d(t,"J",(function(){return S})),e.d(t,"H",(function(){return I})),e.d(t,"C",(function(){return L})),e.d(t,"F",(function(){return A})),e.d(t,"B",(function(){return T})),e.d(t,"g",(function(){return J})),e.d(t,"q",(function(){return N})),e.d(t,"i",(function(){return B})),e.d(t,"p",(function(){return C})),e.d(t,"d",(function(){return j})),e.d(t,"s",(function(){return K})),e.d(t,"r",(function(){return D})),e.d(t,"y",(function(){return P})),e.d(t,"w",(function(){return q})),e.d(t,"o",(function(){return F})),e.d(t,"j",(function(){return U})),e.d(t,"c",(function(){return $})),e.d(t,"k",(function(){return H})),e.d(t,"b",(function(){return R})),e.d(t,"E",(function(){return W})),e.d(t,"m",(function(){return z})),e.d(t,"t",(function(){return G})),e.d(t,"x",(function(){return V})),e.d(t,"e",(function(){return X})),e.d(t,"f",(function(){return Y})),e.d(t,"A",(function(){return Q})),e.d(t,"a",(function(){return Z})),e.d(t,"h",(function(){return nn})),e.d(t,"l",(function(){return tn})),e.d(t,"u",(function(){return en})),e.d(t,"n",(function(){return rn})),e.d(t,"v",(function(){return on})),e.d(t,"z",(function(){return un})),e.d(t,"I",(function(){return cn})),e.d(t,"G",(function(){return ln})),e.d(t,"D",(function(){return fn}));var o=e(5);let u=new("undefined"==typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});u.decode();let i=null;function c(){return null!==i&&i.buffer===o.l.buffer||(i=new Uint8Array(o.l.buffer)),i}function l(n,t){return u.decode(c().subarray(n,n+t))}const f=new Array(32).fill(void 0);f.push(void 0,null,!0,!1);let s=f.length;function d(n){s===f.length&&f.push(f.length+1);const t=s;return s=f[t],f[t]=n,t}function a(n){return f[n]}let p=0;let h=new("undefined"==typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof h.encodeInto?function(n,t){return h.encodeInto(n,t)}:function(n,t){const e=h.encode(n);return t.set(e),{read:n.length,written:e.length}};function b(n,t,e){if(void 0===e){const e=h.encode(n),r=t(e.length);return c().subarray(r,r+e.length).set(e),p=e.length,r}let r=n.length,o=t(r);const u=c();let i=0;for(;i<r;i++){const t=n.charCodeAt(i);if(t>127)break;u[o+i]=t}if(i!==r){0!==i&&(n=n.slice(i)),o=e(o,r,r=i+3*n.length);const t=c().subarray(o+i,o+r);i+=y(n,t).written}return p=i,o}let m=null;function v(){return null!==m&&m.buffer===o.l.buffer||(m=new Int32Array(o.l.buffer)),m}function w(n){const t=a(n);return function(n){n<36||(f[n]=s,s=n)}(n),t}function g(n){try{const i=o.a(-16);var t=b(n,o.d,o.e),e=p;o.q(i,t,e);var r=v()[i/4+0],u=v()[i/4+1];return l(r,u)}finally{o.a(16),o.c(r,u)}}function x(n,t){try{const s=o.a(-16);var e=b(n,o.d,o.e),r=p,u=b(t,o.d,o.e),i=p;o.p(s,e,r,u,i);var c=v()[s/4+0],f=v()[s/4+1];return l(c,f)}finally{o.a(16),o.c(c,f)}}function E(n,t){var e=b(n,o.d,o.e),r=p,u=b(t,o.d,o.e),i=p;return o.k(e,r,u,i)}function _(n,t){try{const s=o.a(-16);var e=b(n,o.d,o.e),r=p,u=b(t,o.d,o.e),i=p;o.t(s,e,r,u,i);var c=v()[s/4+0],f=v()[s/4+1];return l(c,f)}finally{o.a(16),o.c(c,f)}}function M(n,t,e){try{const d=o.a(-16);var r=b(n,o.d,o.e),u=p,i=b(t,o.d,o.e),c=p;o.m(d,r,u,i,c,e);var f=v()[d/4+0],s=v()[d/4+1];return l(f,s)}finally{o.a(16),o.c(f,s)}}function S(n,t,e,r,u){try{const h=o.a(-16);var i=b(n,o.d,o.e),c=p,f=b(t,o.d,o.e),s=p;o.j(h,i,c,f,s,e,r,u);var d=v()[h/4+0],a=v()[h/4+1];return l(d,a)}finally{o.a(16),o.c(d,a)}}function k(n,t){try{return n.apply(this,t)}catch(n){o.b(d(n))}}function O(n,t){return c().subarray(n/1,n/1+t)}function I(n,t){return d(l(n,t))}function L(n,t){const e=a(t);var r=b(JSON.stringify(void 0===e?null:e),o.d,o.e),u=p;v()[n/4+1]=u,v()[n/4+0]=r}function A(n){w(n)}function T(n,t){return d(JSON.parse(l(n,t)))}function J(n){return a(n)instanceof Window}function N(n){var t=a(n).performance;return null==t?0:d(t)}function B(n){console.log(a(n))}function C(n){return a(n).now()}function j(){return k((function(n,t){a(n).getRandomValues(a(t))}),arguments)}function K(){return k((function(n,t,e){a(n).randomFillSync(O(t,e))}),arguments)}function D(n){return d(a(n).process)}function P(n){const t=a(n);return"object"==typeof t&&null!==t}function q(n){return d(a(n).versions)}function F(n){return d(a(n).node)}function U(){return k((function(n,t){return d(e(6)(l(n,t)))}),arguments)}function $(n){return d(a(n).crypto)}function H(n){return d(a(n).msCrypto)}function R(){return k((function(n,t){return d(a(n).call(a(t)))}),arguments)}function W(n){return d(a(n))}function z(n,t){return d(new Function(l(n,t)))}function G(){return k((function(){return d(self.self)}),arguments)}function V(){return k((function(){return d(window.window)}),arguments)}function X(){return k((function(){return d(globalThis.globalThis)}),arguments)}function Y(){return k((function(){return d(r.global)}),arguments)}function Q(n){return void 0===a(n)}function Z(n){return d(a(n).buffer)}function nn(n){return a(n).length}function tn(n){return d(new Uint8Array(a(n)))}function en(n,t,e){a(n).set(a(t),e>>>0)}function rn(n){return d(new Uint8Array(n>>>0))}function on(n,t,e){return d(a(n).subarray(t>>>0,e>>>0))}function un(n){return"string"==typeof a(n)}function cn(n,t){throw new Error(l(n,t))}function ln(n){throw w(n)}function fn(){return d(o.l)}}).call(this,e(3)(n),e(4))},function(n,t){n.exports=function(n){if(!n.webpackPolyfill){var t=Object.create(n);t.children||(t.children=[]),Object.defineProperty(t,"loaded",{enumerable:!0,get:function(){return t.l}}),Object.defineProperty(t,"id",{enumerable:!0,get:function(){return t.i}}),Object.defineProperty(t,"exports",{enumerable:!0}),t.webpackPolyfill=1}return t}},function(n,t){var e;e=function(){return this}();try{e=e||new Function("return this")()}catch(n){"object"==typeof window&&(e=window)}n.exports=e},function(n,t,e){"use strict";var r=e.w[n.i];n.exports=r;e(2);r.v()},function(n,t){function e(n){var t=new Error("Cannot find module '"+n+"'");throw t.code="MODULE_NOT_FOUND",t}e.keys=function(){return[]},e.resolve=e,n.exports=e,e.id=6}]]);
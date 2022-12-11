"use strict";(self.webpackChunkthalo_docs=self.webpackChunkthalo_docs||[]).push([[671],{3905:(e,t,a)=>{a.d(t,{Zo:()=>c,kt:()=>h});var n=a(7294);function l(e,t,a){return t in e?Object.defineProperty(e,t,{value:a,enumerable:!0,configurable:!0,writable:!0}):e[t]=a,e}function r(e,t){var a=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),a.push.apply(a,n)}return a}function o(e){for(var t=1;t<arguments.length;t++){var a=null!=arguments[t]?arguments[t]:{};t%2?r(Object(a),!0).forEach((function(t){l(e,t,a[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(a)):r(Object(a)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(a,t))}))}return e}function i(e,t){if(null==e)return{};var a,n,l=function(e,t){if(null==e)return{};var a,n,l={},r=Object.keys(e);for(n=0;n<r.length;n++)a=r[n],t.indexOf(a)>=0||(l[a]=e[a]);return l}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(n=0;n<r.length;n++)a=r[n],t.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(e,a)&&(l[a]=e[a])}return l}var s=n.createContext({}),p=function(e){var t=n.useContext(s),a=t;return e&&(a="function"==typeof e?e(t):o(o({},t),e)),a},c=function(e){var t=p(e.components);return n.createElement(s.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var a=e.components,l=e.mdxType,r=e.originalType,s=e.parentName,c=i(e,["components","mdxType","originalType","parentName"]),u=p(a),d=l,h=u["".concat(s,".").concat(d)]||u[d]||m[d]||r;return a?n.createElement(h,o(o({ref:t},c),{},{components:a})):n.createElement(h,o({ref:t},c))}));function h(e,t){var a=arguments,l=t&&t.mdxType;if("string"==typeof e||l){var r=a.length,o=new Array(r);o[0]=d;var i={};for(var s in t)hasOwnProperty.call(t,s)&&(i[s]=t[s]);i.originalType=e,i[u]="string"==typeof e?e:l,o[1]=i;for(var p=2;p<r;p++)o[p]=a[p];return n.createElement.apply(null,o)}return n.createElement.apply(null,a)}d.displayName="MDXCreateElement"},9881:(e,t,a)=>{a.r(t),a.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>u,frontMatter:()=>r,metadata:()=>i,toc:()=>p});var n=a(7462),l=(a(7294),a(3905));const r={sidebar_position:1},o="Getting Started",i={unversionedId:"intro",id:"intro",title:"Getting Started",description:"You will need a couple of tools to build and publish wasm components to the runtime.",source:"@site/docs/intro.md",sourceDirName:".",slug:"/intro",permalink:"/thalo/docs/intro",draft:!1,editUrl:"https://github.com/thalo-rs/thalo-docs/tree/main/packages/create-docusaurus/templates/shared/docs/intro.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"tutorialSidebar",next:{title:"Runtime",permalink:"/thalo/docs/category/runtime"}},s={},p=[{value:"Install Thalo Runtime",id:"install-thalo-runtime",level:2},{value:"Install Rust",id:"install-rust",level:2},{value:"Install WebAssembly Target",id:"install-webassembly-target",level:2},{value:"Install wasm-tools",id:"install-wasm-tools",level:2},{value:"Install <code>wasi_snapshot_preview1</code> Adapter",id:"install-wasi_snapshot_preview1-adapter",level:2}],c={toc:p};function u(e){let{components:t,...a}=e;return(0,l.kt)("wrapper",(0,n.Z)({},c,a,{components:t,mdxType:"MDXLayout"}),(0,l.kt)("h1",{id:"getting-started"},"Getting Started"),(0,l.kt)("p",null,"You will need a couple of tools to build and publish wasm components to the runtime."),(0,l.kt)("h2",{id:"install-thalo-runtime"},"Install Thalo Runtime"),(0,l.kt)("p",null,"Thalo runtime can be installed via Docker, or built manually with Cargo."),(0,l.kt)("ul",null,(0,l.kt)("li",{parentName:"ul"},(0,l.kt)("a",{parentName:"li",href:"../runtime/installation-cargo"},"Installation Cargo")),(0,l.kt)("li",{parentName:"ul"},(0,l.kt)("a",{parentName:"li",href:"../runtime/installation-docker-compose"},"Installation Docker Compose"))),(0,l.kt)("h2",{id:"install-rust"},"Install Rust"),(0,l.kt)("p",null,"To install Rust, follow the ",(0,l.kt)("a",{parentName:"p",href:"https://www.rust-lang.org/tools/install"},"official instructions"),"."),(0,l.kt)("h2",{id:"install-webassembly-target"},"Install WebAssembly Target"),(0,l.kt)("p",null,"Rust can compile to different build targets, one being Webassembly called ",(0,l.kt)("inlineCode",{parentName:"p"},"wasm32-wasi"),", which is the build target\nneeded when writing modules for Thalo."),(0,l.kt)("pre",null,(0,l.kt)("code",{parentName:"pre",className:"language-bash"},"rustup target add wasm32-wasi\n")),(0,l.kt)("h2",{id:"install-wasm-tools"},"Install wasm-tools"),(0,l.kt)("p",null,(0,l.kt)("a",{parentName:"p",href:"https://github.com/bytecodealliance/wasm-tools"},"Wasm-tools")," is a cli to create a component from a wasm binary. The Thalo runtime uses these components to\nhandle commands, so we'll need to install the cli."),(0,l.kt)("pre",null,(0,l.kt)("code",{parentName:"pre",className:"language-bash"},"cargo install wasm-tools\n")),(0,l.kt)("p",null,"To validate wasm-tools is installed, you can try running ",(0,l.kt)("inlineCode",{parentName:"p"},"wasm-tools --version"),"."),(0,l.kt)("h2",{id:"install-wasi_snapshot_preview1-adapter"},"Install ",(0,l.kt)("inlineCode",{parentName:"h2"},"wasi_snapshot_preview1")," Adapter"),(0,l.kt)("p",null,"When creating a wasm component using the ",(0,l.kt)("a",{parentName:"p",href:"https://github.com/bytecodealliance/wasm-tools"},"wasm-tools")," cli, it requires an adapter module used to translate the\n",(0,l.kt)("inlineCode",{parentName:"p"},"wasi_snapshot_preview1")," ABI, for example, to one that uses the component model."),(0,l.kt)("p",null,"At the time of writing, an adapter module is being actively developed by the BytecodeAlliance team in the\n",(0,l.kt)("a",{parentName:"p",href:"https://github.com/bytecodealliance/preview2-prototyping"},"bytecodealliance/preview2-prototyping")," repository on GitHub.\nWe can use this adapter when creating wasm componens for Thalo."),(0,l.kt)("p",null,"Head to the releases page in the ",(0,l.kt)("a",{parentName:"p",href:"https://github.com/bytecodealliance/preview2-prototyping/releases"},"preview2-prototyping")," repository and download the ",(0,l.kt)("inlineCode",{parentName:"p"},"wasi_snapshot_preview1.wasm")," file."),(0,l.kt)("p",null,(0,l.kt)("img",{parentName:"p",src:"/img/wasi_snapshot_preview1-download.png",alt:"wasi_snapshot_preview1 release download"})),(0,l.kt)("p",null,"Save this file, and take note of its location, as it'll be used later."))}u.isMDXComponent=!0}}]);
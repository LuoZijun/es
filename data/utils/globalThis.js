
// https://github.com/tc39/proposal-global
if (typeof globalThis === 'undefined') {
    var globalThis = undefined;
    
    if ( typeof globalThis === 'undefined' && typeof window !== 'undefined' ) {
        // `window` used in browsers.
        globalThis = window;
    }

    if ( typeof globalThis === 'undefined' && typeof global !== 'undefined' ) {
        // `global` used in node
        globalThis = global;
    }

    if ( typeof globalThis === 'undefined' && typeof top !== 'undefined' ) {
        // `top` used in browsers.
        globalThis = top;
    }

    if ( typeof globalThis === 'undefined' && typeof self !== 'undefined' ) {
        // `self` used in browsers.
        globalThis = self;
    }

    if ( typeof globalThis === 'undefined' && typeof GLOBAL !== 'undefined' ) {
        // `GLOBAL` used in node.
        globalThis = GLOBAL;
    }

    if ( typeof globalThis === 'undefined' && typeof root !== 'undefined' ) {
        // `root` used in node.
        globalThis = root;
    }

    if ( typeof globalThis === 'undefined' && typeof this !== 'undefined' ) {
        globalThis = this;
    }

    if ( typeof globalThis === 'undefined' ) {
        throw new Error("RunTime Error.");
    }
}

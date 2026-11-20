# `ofx-ls-plugins`

List OpenFX Image Effect Version 1 plugins.

## Usage

```sh
ofx-ls-plugins $SOMEWHERE/Plugins
ofx-ls-plugins --mode=auto $SOMEWHERE/Plugins # same as above
ofx-ls-plugins --mode=flat-folder $SOMEWHERE/Plugins # same as above

ofx-ls-plugins $SOMEWHERE/Plugins/Foo.ofx.bundle
ofx-ls-plugins --mode=bundle $SOMEWHERE/Plugins/Foo.ofx.bundle # same as above

ofx-ls-plugins $SOMEWHERE/Plugins/Foo.ofx.bundle/Contents/MacOS/Foo.ofx
ofx-ls-plugins --mode=binary $SOMEWHERE/Plugins/Foo.ofx.bundle/Contents/MacOS/Foo.ofx # same as above
```

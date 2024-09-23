## `swc-plugin-accuracy` 
swc-loader 插件 提供处理js 计算精度丢失的统一方案 , 同时提供
- async函数体添加try-catch功能
- js  === 严格等于处理方案
- promise 默认添加catch处理
- date参数处理

### 计算精度丢失
内部提供加、减、乘、除方法。
🌰
before
```
function a(b, c) {
    const d = 0.1 + 0.4
    if (b === c) {
        return 0
    }
}

```
after
```
const { accAdd, accCong } = require("babel-plugin-accuracy/src/calc.js");

function a(b, c) {
    const d = accAdd(0.1, 0.4);

    if (accCong(b, c)) {
        return 0;
    }
}
```

### async函数添加try-catch
配置: addAsyncTry。默认为false。
如果async函数中已经被try-catch处理，则不会在添加。
🌰

before
```
async function printFile(filename) {
    let contents = await fs.readFileAsync(filename, 'utf8');
    console.log(contents);
}
async (filename) => {
    let contents = await fs.readFileAsync(filename, 'utf8');
    console.log(contents);
}
```
after
```
async function printFile(filename) {
    try {
        let contents = await fs.readFileAsync(filename, 'utf8');
        console.log(contents);
    } catch (error) {
        console.error(this, error);
    }
}

async filename => {
    try {
        let contents = await fs.readFileAsync(filename, 'utf8');
        console.log(contents);
    } catch (error) {
        console.error(this, error);
    }
};

```

### promise最后添加catch
配置：promiseCatch 。 默认为false
如果promise调用链中调用了catch，则不会在添加
🌰
before
```
a.then(function (resolve) {
    resolve()
}).then(()=>{
    return 0
})
```
after
```
a.then(function(resolve) {
    resolve();
}).then(() => {
    return 0;
}).catch((err) => {
    console.error(err);
})
```

### js === 严格等于
配置: checkChong 默认为false
🌰
before
```
function a(b, c) {
    const d = 0.1 + 0.4
    if (b === c) {
        return 0
    }
}
```
after
```
const { accSub, accCong } = require("babel-plugin-accuracy/src/calc.js");

function a(b, c) {
    const d = accSub(0.1, 0.4);

    if (accCong(b, c)) {
        return 0;
    }
}

```

### date参数处理
🌰
before
```
new Date('1982-12-2')
```
after
```
new Date("1982-12-2".replace(/-/g, "/"))
```


### 在.swcrc中的完成配置使用
```
 {
    "jsc": {
      "parser": {
        "syntax": "typescript",
        "tsx": true,
        "dynamicImport": true,
        "privateMethod": true,
        "functionBind": true,
        "exportNamespaceFrom": true,
        "decorators": true,
        "classProperties": true
      },
      "transform": {
        "legacyDecorator": true,
        "decoratorMetadata": true,
      },
      "loose": true,
      "experimental": {
        "plugins": [
          [
            "swc-plugin-accuracy",
            {
              "checkChong": false,
              "addAsyncTry": true,
              "promiseCatch": true
            }
          ]
        ]
      }
    },
    "env": {
      "targets": "> 1%, last 2 versions, not ie <= 8",
      "mode": "usage",
      "coreJs": 3
    },
}
```




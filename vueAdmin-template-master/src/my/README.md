## How to build

### Get the latest source

```bash
wget -qO- https://github.com/PanJiaChen/vueAdmin-template/archive/master.zip | bsdtar -xvf-
cd vueAdmin-template
npm install --save graphql-request js-cookie jwt-decode moment moment-timezone
```

### Files need to merge

-   .gitignore
-   src/my

### Create src/main.js

```js
import main from './my'
main('#app')
```

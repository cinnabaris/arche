import '@babel/polyfill';
import 'url-polyfill';

import registerServiceWorker from './registerServiceWorker'
import main from './main'

main('root')
registerServiceWorker()

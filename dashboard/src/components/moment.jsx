import moment from 'moment'
import 'moment-timezone'
import 'moment/locale/zh-cn'
import 'moment/locale/zh-tw'
import 'moment/locale/en-gb'

import {get as detectLocale} from '../intl'
moment.locale(detectLocale().moment)

var TZ = moment.tz.guess();

// https://momentjs.com/docs/#/displaying/format/
export const timestamp = (value) => moment.utc(value).tz(TZ).format('llll z')

export const timeago = (value) => moment.utc(value).fromNow()

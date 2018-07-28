import moment from 'moment'

export const ago = (v) => moment(v).fromNow()

export const show = (v) => moment(v).format('LLLL')

export const DATE_FORMAT = 'YYYY-MM-DD'
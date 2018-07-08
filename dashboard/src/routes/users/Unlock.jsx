import React from 'react'

import EmailForm from './EmailForm'
import {USERS_UNLOCK} from '../../request'

const Widget = () => (<EmailForm action="unlock" query={USERS_UNLOCK}/>)

export default Widget

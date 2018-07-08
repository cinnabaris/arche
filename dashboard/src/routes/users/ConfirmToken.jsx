import React from 'react'

import Token from './Token'
import {USERS_CONFIRM_TOKEN} from '../../request'

const Widget = () => (<Token action="confirm" query={USERS_CONFIRM_TOKEN}/>)

export default Widget

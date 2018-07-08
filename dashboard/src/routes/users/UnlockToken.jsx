import React from 'react'

import Token from './Token'
import {USERS_UNLOCK_TOKEN} from '../../request'

const Widget = () => (<Token action="unlock" query={USERS_UNLOCK_TOKEN}/>)

export default Widget

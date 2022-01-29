import 'regenerator-runtime/runtime'
import React, { useState } from 'react'
import { logout } from './utils'
import { LoginPane } from './Login'
import './global.css'

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')


export default function App() {

  const [value, setValue] = useState(undefined);

  console.log(window.contract)

  React.useEffect(
    () => {
      if (window.walletConnection.isSignedIn()) {
        window.contract.show_value({}).then(value => {setValue(value)})
      }
    },


    []
  )

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {return <LoginPane></LoginPane>}

  return <main>
    <h1> Some text</h1>
    <button onClick={() => logout()}>Logout</button>
  </main  >
}
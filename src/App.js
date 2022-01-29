import 'regenerator-runtime/runtime'
import React, { useState } from 'react'
import { logout } from './utils'
import { LoginPane } from './Login'
import './global.css'

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {

  console.log(window.contract)
  const [value, setValue] = useState(undefined);

  const updateValue = () => {
    window.contract.show_value({}).then(setValue)
  }

  React.useEffect(
    () => {
      if (window.walletConnection.isSignedIn()) {
        window.contract.show_value({}).then(value => { setValue(value) })
      }
    }, [])

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) { return <LoginPane></LoginPane> }

  return <main>

    <p style={{ textAlign: 'right' }}>
      <button onClick={logout}>Log Out</button>
    </p>
    <h1> Some text {value} </h1>

    <p>
      <button onClick={() => window.contract.inc_value().then(_ => updateValue())}>Increment Value</button>
    </p>
    <p>
      <button onClick={() => window.contract.reset_value().then(_ => updateValue()).catch(e => alert(e))}>Reset Value (Only Owner)</button>

    </p>

  </main  >
}
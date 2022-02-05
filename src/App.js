import 'regenerator-runtime/runtime'
import React, { useEffect, useState } from 'react'
import { logout } from './utils'
import { LoginPane } from './Login'
import { ContributeGiftPane } from './ContributeGIftPane'
import { AddGiftPane } from './AddGiftPane'
import './global.css'

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {

  const [mode, setMode] = useState('Add Gifts');

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) { return <LoginPane></LoginPane> }

  return <main>
    <div className="navbar">
      <button onClick={() => setMode('Add Gifts')}>Add Gifts</button>
      <button onClick={() => setMode('Contribute Gifts')}>Contribute Gifts</button>
      <button onClick={() => logout()}> Log Out</button>
    </div>

    {(mode === 'Add Gifts') && <AddGiftPane></AddGiftPane>}
    {(mode === 'Contribute Gifts') && <ContributeGiftPane></ContributeGiftPane>}

  </main  >
}
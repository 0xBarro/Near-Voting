import 'regenerator-runtime/runtime'
import React, { useEffect, useState } from 'react'
import { logout } from './utils'
import { LoginPane } from './Login'
import { ContributeGiftPane } from './ContributeGIftPane'
import { AddGiftPane } from './AddGiftPane'
import { GiftsList } from './GiftsList'
import './global.css'

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {

  const [mode, setMode] = useState('Add Gifts');

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) { return <LoginPane></LoginPane> }

  return <main>
    <div className="navbar">
      <button className={(mode === "My Gifts") ? 'selected-button' : ""} onClick={() => setMode('My Gifts')}>My Gifts</button>
      <button className={(mode === "Add Gifts") ? 'selected-button' : ""} onClick={() => setMode('Add Gifts')}>Add Gifts</button>
      <button className={(mode === "Contribute Gifts") ? 'selected-button' : ""} onClick={() => setMode('Contribute Gifts')}>Contribute Gifts</button>
      <button className='salmon-bg' onClick={() => logout()}> Log Out</button>
    </div>

    <h3> Logged in as {window.accountId}</h3>

    {(mode === 'My Gifts') && <GiftsList account={window.accountId}></GiftsList>}
    {(mode === 'Add Gifts') && <AddGiftPane></AddGiftPane>}
    {(mode === 'Contribute Gifts') && <ContributeGiftPane></ContributeGiftPane>}

  </main  >
}
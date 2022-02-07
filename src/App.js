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

  const [mode, setMode] = useState('Add Item');

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) { return <LoginPane></LoginPane> }

  return <main>
    <div className="navbar">
      <button className={(mode === "My Wishlist") ? 'selected-button' : ""} onClick={() => setMode('My Wishlist')}>My Wishlist</button>
      <button className={(mode === "Add Item") ? 'selected-button' : ""} onClick={() => setMode('Add Item')}>Add Item</button>
      <button className={(mode === "Contribute") ? 'selected-button' : ""} onClick={() => setMode('Contribute')}>Contribute</button>
      <button className={(mode === "About") ? 'selected-button' : ""} onClick={() => setMode('About')}>About</button>
      <button className='salmon-bg' onClick={() => logout()}> Log Out</button>
    </div>

    <h3> Logged in as {window.accountId}</h3>

    {(mode === 'My Wishlist') && <GiftsList account={window.accountId}></GiftsList>}
    {(mode === 'Add Item') && <AddGiftPane></AddGiftPane>}
    {(mode === 'Contribute') && <ContributeGiftPane></ContributeGiftPane>}
    {(mode === 'About') && <div>
      <p> This app allows the users to create a wishlist of online shopping items. Users can then get contributions and contribute to other's wishlists. </p>
      </div>
      }

  </main  >
}
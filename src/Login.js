import { login } from './utils'
import React, { useState } from 'react'

export const LoginPane = () => {
    return <main>
        <h1>Decentralized Voting</h1>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
            <p>Login to your wallet to vote!</p>
            <button onClick={login}>Sign in</button>
        </p>
    </main>
} 
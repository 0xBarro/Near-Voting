import React, { useState } from 'react'

export const SelectAccount = props => {
    const [account, setAccount] = useState(undefined);

    const onChangeHandler = (v) => {
        const value = v.target.value;
        setAccount(value);
    }

    return <div>
        <h1>Please select an account to view the wishlist.</h1>
        <input type="text" onChange={onChangeHandler}></input>
        <button onClick={() => props.setAccount(account)}>Submit</button>
    </div>

}
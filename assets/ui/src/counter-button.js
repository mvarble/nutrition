import React from 'react';

export default function CounterButton({ amount, setAmount }) {
  const [text, setText] = React.useState(`${amount || ''}`);

  const onChange = React.useCallback(
    e => setText(e.target.value),
    [setText],
  );

  const onBlur = React.useCallback(e => {
    const text = e.target.value;
    const parsed = +text;
    if (Number.isFinite(parsed) && parsed > 0) {
      setAmount(parsed);
    }  else {
      setText(`${amount}`);
    }
  }, [setAmount, setText, amount]);

  const dec = () => {
    const newAmount = amount - 1 > 0 ? amount - 1 : amount;
    setText(`${newAmount}`)
    setAmount(newAmount);
  };

  const inc = () => {
    const newAmount = amount + 1;
    setText(`${newAmount}`)
    setAmount(newAmount);
  };

  return (
    <div className="field has-addons">
      <p className="control">
        <a className="button is-info" onClick={ dec }>-</a>
      </p>
      <p className="control">
        <input 
          className="input" 
          value={ text } 
          onChange={ onChange } 
          onBlur={ onBlur } />
      </p>
      <p className="control">
        <a className="button is-info" onClick={ inc }>+</a>
      </p>
    </div>
  )
}

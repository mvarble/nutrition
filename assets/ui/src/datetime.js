import React from 'react';

function optISODate(str) {
  if (!/\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d{3}Z/.test(str)) return false;
  const d = new Date(str);
  return d.toISOString()===str ? d : null;
}

export default function Datetime({ datetime, setDatetime }) {
  const [text, setText] = React.useState(datetime.toISOString());
  const onBlur = () => {
    const dt = optISODate(text);
    if (dt) {
      setDatetime(dt);
    } else {
      setText(datetime.toISOString());
    }
  };
  const onNow = () => {
    const d = new Date();
    setDatetime(d);
    setText(d.toISOString());
  };
  return (
    <div className="field has-addons">
      <div className="control" style={{ flexGrow: 1 }}>
        <input 
          className="input" 
          value={ text }
          onChange={ e => setText(e.target.value) } 
          onBlur={ onBlur } />
      </div>
      <div className="control">
        <a className="button is-info" onClick={ onNow }>
          Now
        </a>
      </div>
    </div>
  );
}

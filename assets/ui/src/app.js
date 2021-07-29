import React from 'react';
import { fromEvent } from 'rxjs';
import { debounceTime, map, distinct } from 'rxjs/operators';
import fp from 'lodash/fp';

function MealForm({ initMeal }) {
  const [meal, setMeal] = React.useState({
    save: fp.getOr(false)('save')(initMeal),
    name: fp.getOr('')('name')(initMeal),
    datetime: fp.getOr(new Date())('datetime')(initMeal),
    totalServings: fp.getOr(1)('totalServings')(initMeal),
    servingsConsumed: fp.getOr(1)('servingsConsumed')(initMeal),
    foodQuantities: fp.getOr([])('foodQuantities')(initMeal),
    id: fp.getOr(null)('id')(initMeal),
  });
  const [
    toggleSave,
    setName,
    setDatetime,
    setTotalServings,
    setServingsConsumed,
    updateFood,
  ] = React.useMemo(() => [
    () => setMeal(fp.update('save')(s=>!s)),
    name => setMeal(fp.set('name')(name)),
    dt => setMeal(fp.set('datetime')(dt)),
    total => setMeal(fp.set('totalServings')(total)),
    consumed => setMeal(fp.set('servingsConsumed')(consumed)),
    index => (callback => setMeal(fp.update(`foodQuantities[${index}]`)(callback))),
  ], [setMeal]);

  return (
    <div>
      <div>
        <span>Saved?</span>
        <input type="checkbox" value={ meal.save } onClick={ toggleSave } />
      </div>
      <div style={{ display: meal.save ? 'block' : 'none' }}>
        <span>Name</span>
        <input value={ meal.name } onChange={ e => setName(e.target.value) } />
      </div>
      <div>
        <span>Datetime</span>
      </div>
      <div>
        <span>Total Servings</span>
        <CounterButton amount={ meal.totalServings } setAmount={ setTotalServings } />
      </div>
      <div>
        <span>Servings Consumed</span>
        <CounterButton amount={ meal.servingsConsumed } setAmount={ setServingsConsumed } />
      </div>
      <div>
        { 
          meal.foodQuantities.map((foodQuant, i) => 
            <FoodQuantityItem 
              setFoodQuantity={ updateFood(i) } 
              food={ foodQuant.food } 
              quantity={ foodQuant.quantity } />
          ) 
        }
        <div>Add Food Quantity</div>
      </div>
      <pre>{ JSON.stringify(meal, null, 2) }</pre>
    </div>
  );
}

function CounterButton({ amount, setAmount }) {
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
    <span>
      <button onClick={ dec }>-</button>
      <input value={ text } onChange={ onChange } onBlur={ onBlur } />
      <button onClick={ inc }>+</button>
    </span>
  )
}

function prepareMealPayload(meal) {
  const updated = fp.update('foodQuantities')(fqs => fqs.map(foodQuant => ({ 
    id: foodQuant.food.id,
    measurement_name: foodQuant.quantity.name,
    amount: foodQuant.quantity.amount,
  })))(meal);
  return JSON.stringify(updated);
}

function FoodQuantityItem({ food, setFoodQuantity, quantity }) {
  return null;
}

function FoodNameSearch({ foodClick }) {
  // create a reference for the input box, as we will need dom-event API
  const ref = React.useRef();

  // create a state of the pending suggestions from the API
  const [suggestions, setSuggestions] = React.useState([]);

  // subscribe api fetches and state updates to the user's running input
  React.useEffect(() => {
    // if there is no ref, return
    const elm = ref.current;
    if (!elm) return;

    // create a stream for debounced field changes
    const query$ = fromEvent(elm, 'keydown').pipe(
      debounceTime(250), 
      map(x => x.target.value),
      distinct(),
    );

    // subscribe api fetches to this stream
    const subscription = query$.subscribe(text => {
      // if the text is empty, don't even perform a fetch
      if (!text) {
        setSuggestions([]);
      } else {
        fetch('/api/v1/foods', {
          method: 'POST',
          mode: 'same-origin',
          headers: {
            'Content-Type': 'application/json',
          },
          body: `{ "query": "${text}" }`,
        })
          .then(res => res.json())
          .then(data => setSuggestions(data.foods))
      }
    });

    // declare the resource cleanup
    const cleanup = () => subscription.unsubscribe();
    return cleanup;
  }, [ref, setSuggestions])

  // need value state
  const [text, setText] = React.useState('');

  // render
  return (
    <div>
      <input 
        ref={ ref } 
        value={ text } 
        onChange={ e => setText(e.target.value) } />
      <div>
        { 
          suggestions.map(sug => 
            <div 
              key={ sug.name }  
              onClick={ () => foodClick(sug) } >
              { sug.name }
            </div>
          ) 
        }
      </div>
    </div>
  );
}

function QuantityForm({ food, quantity }) {
  // reduce through the measurements of the food prop, stratifying by SI type
  const measurements = React.useMemo(() => (
    fp.flow([
      fp.getOr([])('measurements'),
      fp.groupBy(m => m.measurement_type),
      fp.update('Mass')(fp.defaultTo([])),
      fp.update('Volume')(fp.defaultTo([])),
      fp.update('Nominal')(fp.defaultTo([])),
    ])(food)
  ), [food]);

  // create an internal state for the selected quantity
  const amount = fp.get('amount')(quantity);
  const [quant, setQuant] = React.useState({ 
    measurement: fp.get('measurement')(quantity) || null,
    amount: (Number.isFinite(amount) && amount > 0) ? amount : NaN,
    amountText: (Number.isFinite(amount) && amount > 0) ? `${amount}` : '', 
  });
  const setMeas = React.useCallback(
    measurement => setQuant(quantity => ({ ...quantity, measurement })),
    [setQuant],
  );
  const setAmnt = React.useCallback(
    amountText => setQuant(quantity => {
      const parsed = +amountText;
      return { 
        ...quantity,
        amountText, 
        amount: (Number.isFinite(parsed) && parsed > 0) ? parsed : NaN
      }
    }),
    [setQuant]
  );

  return (
    <div>
      <h1>Measurements</h1>
      <div>
        { 
          ['Mass', 'Volume', 'Nominal'].map(name => (
            measurements[name].length
            ? <MeasurementSelector 
              key={ name }
              title={ name } 
              measurement={ fp.get('measurement')(quant) }
              measurements={ measurements[name] } 
              onSelect={ setMeas } />
            : null
          ))
        }
      </div>
        <input 
          value={ quant.amountText } 
          onChange={ e => setAmnt(e.target.value) } />
        <p>
          { food && quant && quant.measurement && quant.amount ? 'VALID' : 'NOT VALID' }
        </p>
        <pre>{ JSON.stringify({ food, quantity: quant }, null, 2) }</pre>
    </div>
  );
}

function MeasurementSelector({ title, measurement, measurements, onSelect }) {
  return (
    <div>
      <h2>{ title }</h2>
      { 
        measurements.map(meas => 
          <button 
            style={{ 
              background: 
                fp.get('name')(measurement) == fp.get('name')(meas) 
                ? '#efeffa' 
                : '#fafafa' 
            }}
            key={ meas.name } 
            onClick={ () => onSelect(meas) }>
            { meas.name }
          </button>
        ) 
      }
    </div>
  );
}

export default function App() {
  let [food, setFood] = React.useState(null);
  return (
    <div>
      <MealForm />
    </div>
  );
}

import React from 'react';
import fp from 'lodash/fp';
import { fromEvent } from 'rxjs';
import { debounceTime, map } from 'rxjs/operators';

import CounterButton from '../counter-button';
import Datetime from '../datetime';
import  {
  current,
  after,
  before,
  fqItem,
  listInput,
} from './index.module.scss';

export default function EditMeal({ initMeal }) {
  // state: this holds the complete details of the meal
  const [meal, setMeal] = React.useState({
    save: fp.getOr(false)('save')(initMeal),
    name: fp.getOr('')('name')(initMeal),
    datetime: fp.getOr(new Date())('datetime')(initMeal),
    totalServings: fp.getOr(1)('totalServings')(initMeal),
    servingsConsumed: fp.getOr(1)('servingsConsumed')(initMeal),
    foodQuantities: fp.getOr([])('foodQuantities')(initMeal),
  });

  // state: this holds a pending foodQuantity which may populate the meal
  const [foodQuantity, setFoodQuantity] = React.useState(null);
  const [foodQuantitySetter, setSetter] = React.useState(() => {});
  const setIndex = React.useCallback(i => (
    setSetter(() => (fq => setMeal(fp.set(`foodQuantities[${i}]`)(fq))))
  ), [setMeal, setSetter]);

  // state: which page of the form we are on
  const [page, setPage] = React.useState(0);
  const [prevPage, setPrevPage] = React.useState(0);

  // render
  return (
    <div>
      <div>
        <MealForm 
          meal={ meal } 
          setMeal={ setMeal } 
          setIndex={ setIndex }
          setPage={ setPage }
          setPrevPage={ setPrevPage }
          setFoodQuantity={ setFoodQuantity }
          className={ page === 0 ? current : before }/>
        <AddFoodForm
          setFoodQuantity={ setFoodQuantity }
          setPage={ setPage }
          className={
            page === 1
            ? current
            : (page < 1 ? after : before)
          }/>
        <QuantityForm
          food={ fp.get('food')(foodQuantity) }
          quantity={ fp.get('quantity')(foodQuantity) }
          setFoodQuantity={ setFoodQuantity }
          foodQuantitySetter={ foodQuantitySetter }
          setPage={ setPage }
          prevPage={ prevPage }
          className={
            page === 2
            ? current
            : (page < 2 ? after : before)
          }/>
      </div>
      <div style={{ maxHeight: '300px', overflowY: 'auto' }}>
        <pre>{ JSON.stringify({ meal, foodQuantity }, null, 2) }</pre>
      </div>
    </div>
  );
}

function MealForm({ 
  meal, 
  setMeal, 
  setIndex, 
  setPage, 
  setPrevPage,
  setFoodQuantity, 
  ...props
}) {
  // state reducers: memoize some callbacks which update parts of the state
  const [
    toggleSave,
    setName,
    setDatetime,
    setTotalServings,
    setServingsConsumed,
  ] = React.useMemo(() => [
    () => setMeal(fp.update('save')(s=>!s)),
    name => setMeal(fp.set('name')(name)),
    dt => setMeal(fp.set('datetime')(dt)),
    s => setMeal(fp.set('totalServings')(s)),
    s => setMeal(fp.set('servingsConsumed')(s)),
  ], [setMeal]);

  const foodCount = meal.foodQuantities.length;

  const updateFood = (fq, i) => {
    setFoodQuantity(fq);
    setIndex(i);
    setPage(page => page + 2);
    setPrevPage(0);
  };

  const addFood = React.useCallback(() => {
    setIndex(foodCount);
    setPage(page => page + 1);
    setPrevPage(1);
  }, [foodCount, setIndex, setPrevPage]);

  return (
    <div { ...props }>
      <div className="section">
        <h2>Edit Meal</h2>
        <div className="field">
          <label className="checkbox">
            <input 
              style={{ margin: '1em' }} 
              type="checkbox" 
              value={ meal.save } 
              onClick={ toggleSave } />
            <span>Save for later use</span>
          </label>
        </div>
        <div 
          style={{ display: meal.save ? 'flex' : 'none' }} 
          className="field is-horizontal">
          <div className="field-label is-normal">
            <label className="label">Name</label>
          </div>
          <div className="field-body" >
            <div className="field">
              <p className="control">
                <input 
                  className="input" 
                  value={ meal.name } 
                  onChange={ e => setName(e.target.value) } />
              </p>
            </div>
          </div>
        </div>
        <div className="field is-horizontal">
          <div className="field-label is-normal">
            <label className="label">Datetime</label>
          </div>
          <div className="field-body" >
            <Datetime datetime={ meal.datetime } setDatetime={ setDatetime } />
          </div>
        </div>
        <div className="field is-horizontal">
          <div className="field-label is-normal">
            <label className="label">Total Servings</label>
          </div>
          <div className="field-body" >
            <CounterButton 
              amount={ meal.totalServings } 
              setAmount={ setTotalServings } />
          </div>
        </div>
        <div className="field is-horizontal">
          <div className="field-label is-normal">
            <label className="label">Servings Consumed</label>
          </div>
          <div className="field-body" >
            <CounterButton 
              amount={ meal.servingsConsumed } 
              setAmount={ setServingsConsumed } />
          </div>
        </div>
        <article className="panel is-info" style={{ margin: '1em 0' }} >
          <div className="panel-heading">
            Ingredients
          </div>
          { 
            meal.foodQuantities.map((foodQuantity, i) => (
              <FoodQuantityItem 
                key={ `${foodQuantity.food.name}${i}` }
                onClick={ () => updateFood(foodQuantity, i) } 
                foodQuantity={ foodQuantity } />
            ))
          }
          <a 
            className="panel-block" 
            onClick={ addFood } 
            style={{ padding: '1em' }}>
            Add ingredient
          </a>
        </article>
      </div>
    </div>
  );
}

function FoodQuantityItem({ foodQuantity, ...props }) {
  return (
    <a className={ `${fqItem} panel-block` } { ...props }>
      <div>
        <span>{ foodQuantity.food.name }</span>
        <span>
          <span>
            { foodQuantity.quantity.amount }
          </span>
          <span>{ foodQuantity.quantity.measurement.name }</span>
          <a className="is-danger"><i class="fas fa-trash" /></a>
        </span>
      </div>
    </a>
  );
}

function AddFoodForm({ setFoodQuantity, setPage, ...props }) {
  // create a reducer which sets defaults for foodQuantity
  const setFood = React.useCallback(food => {
    const quantity = { measurement: { name: null, mass: NaN }, amount: 1.0 };
    setFoodQuantity(fp.set('food')(food)({ quantity }));
    setPage(page => page + 1);
  }, [setFoodQuantity]);

  return (
    <div { ...props }>
      <div className="section">
        <h2>Add food</h2>
        <FoodNameSearch setFood={ setFood } />
        <UPCSearch setFood={ setFood } />
        <button className="button is-info" onClick={ () => setPage(p=>p-1) }>
          Back
        </button>
      </div>
    </div>
  );
}

function FoodNameSearch({ setFood }) {
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

  // quick callback
  const onClick = suggestion => (() => {
    setFood(suggestion);
    setText('');
    setSuggestions([]);
  });

  // render
  return (
    <div className="field is-horizontal">
      <div className="field-label is-normal">
        <label className="label">Food Name</label>
      </div>
      <div className={ `${listInput} field-body` }>
        <input 
          ref={ ref } 
          className="input"
          value={ text } 
          onChange={ e => setText(e.target.value) }>
        </input>
        <div className="panel">
          { 
            suggestions.map(sug => 
              <a
                className="panel-block"
                key={ sug.name } 
                onClick={ onClick(sug) }>
                { sug.name }
              </a>
            ) 
          }
        </div>
      </div>
    </div>
  );
}

function UPCSearch({ setFood }) {
  // state: text for upc
  const [text, setText] = React.useState('');

  // whether current state is valid
  const valid = /\d{12}/.test(text) && text.length === 12;

  return (
    <div className="field is-horizontal">
      <div className="field-label is-normal">
        <label className="label">UPC</label>
      </div>
      <div className="field-body">
        <div className="field has-addons">
          <p className="control">
            <input 
              className="input" 
              style={{ flexGrow: 1 }}
              value={ text } 
              onChange={ e => setText(e.target.value) } />
          </p>
          <p className="control">
            <a className="button is-info">
              <span className="icon">
                <i className="fas fa-barcode" />
              </span>
            </a>
          </p>
          <p className="control">
            <button className="button is-info" disabled={ !valid }>
              Search
            </button>
          </p>
        </div>
      </div>
    </div>
  )
}

function QuantityForm({ 
  food, 
  quantity, 
  setFoodQuantity, 
  foodQuantitySetter, 
  setPage, 
  prevPage,
  ...props
}) {
  // reduce through the measurements of the food prop, stratifying by SI type
  const measurements = React.useMemo(() => (
    fp.flow([
      fp.getOr([])('measurements'),
      fp.groupBy(m => m.measurement_type),
      fp.update('Mass')(fp.defaultTo([])), fp.update('Volume')(fp.defaultTo([])),
      fp.update('Nominal')(fp.defaultTo([])),
    ])(food)
  ), [food]);

  // create the reducers for the quantity
  const amount = fp.get('amount')(quantity) || 1.0;
  const setAmount = a => setFoodQuantity(fp.set('quantity.amount')(a));
  const setMeas = m => setFoodQuantity(fp.set('quantity.measurement')(m));

  // create the reducer for confirmation
  const confirm = React.useCallback(() => {
    foodQuantitySetter({ food, quantity });
    setFoodQuantity(null);
    setPage(0);
  }, [food, quantity, foodQuantitySetter, setFoodQuantity, setPage]);

  // render the form for quantity selection
  return (
    <div { ...props }>
      <div className="section">
        <h2>Select quantity for <span style={{ fontStyle: 'italic' }}>
          { fp.get('name')(food) }
        </span></h2>
        { 
          ['Mass', 'Volume', 'Nominal'].map(name => (
            measurements[name].length
            ? <MeasurementSelector 
              key={ name }
              title={ name } 
              measurement={ fp.get('measurement')(quantity) }
              measurements={ measurements[name] } 
              onSelect={ setMeas } />
            : null
          ))
        }
        <div className="field is-horizontal">
          <div className="field-label is-normal">
            <label className="label">Amount</label>
          </div>
          <div className="field-body" >
            <CounterButton amount={ amount } setAmount={ setAmount } />
          </div>
        </div>
        <button 
          style={{ marginRight: '1em' }} 
          className="button is-info" 
          onClick={ () => setPage(prevPage) }>
          Back
        </button>
        <button 
          className="button is-info" 
          onClick={ confirm } 
          disabled={ 
            !fp.get('measurement.name')(quantity) 
              || !fp.get('measurement.mass')(quantity) 
          }>
          Confirm
        </button>
      </div>
    </div>
  );
}

function MeasurementSelector({ title, measurement, measurements, onSelect }) {
  return (
    <div style={{ margin: '1em 0' }} >
      <h4>{ title }</h4>
      { 
        measurements.map(meas => {
          const selected = fp.get('name')(measurement) == fp.get('name')(meas);
          return (
            <button
              style={{ margin: '0.5em' }}
              key={ meas.name } 
              className={ `button is-${selected ? 'info' : 'light'}` }
              onClick={ () => onSelect(meas) }>
              { meas.name }
            </button>
          );
        })
      }
    </div>
  );
}

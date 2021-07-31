import React from 'react';
import { fromEvent } from 'rxjs';
import { debounceTime, map } from 'rxjs/operators';
import fp from 'lodash/fp';
import shallow from 'zustand/shallow';

import EditMeal from './edit-meal';

export default function App() {
  return (
    <div className="is-marginless is-paddingless">
      <div className="section">
        <div className="columns">
          <div className="column is-two-thirds-widescreen is-full-desktop" style={{ margin: '0 auto' }}>
            <div className="content">
              <EditMeal />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

import React, { FC } from 'react';
import { PlusIcon } from '@primer/octicons-react';
import './styles.css';

const AddPhoto: FC = () => (
  <button className="add-photo">
    <h1>Add photo.</h1>
    <PlusIcon />
  </button>
);

export default AddPhoto;

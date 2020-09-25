import React, { FC, useCallback, useRef, ChangeEvent } from 'react';
import { PlusIcon } from '@primer/octicons-react';
import './styles.css';

const AddPhoto: FC = () => {
  const inputRef = useRef<HTMLInputElement | null>(null);

  const handleClick = useCallback(() => {
    if (!inputRef.current) return;

    inputRef.current.click();
  }, []);

  const handleFileChange = useCallback(() => {
    if (!inputRef.current) return;

    console.log('file', inputRef.current.files);
  }, []);

  return (
    <>
      <button className="add-photo" onClick={handleClick}>
        <h1>Add photo</h1>
        <PlusIcon size="medium" />
      </button>
      <input ref={inputRef} type="file" className="add-photo__input" onChange={handleFileChange} />
    </>
  );
};

export default AddPhoto;

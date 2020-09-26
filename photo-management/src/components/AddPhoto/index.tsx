import React, { FC, useCallback, useRef, useState } from 'react';
import { PlusIcon } from '@primer/octicons-react';
import './styles.css';

const AddPhoto: FC = () => {
  const inputRef = useRef<HTMLInputElement | null>(null);
  const [imagePreview, setImagePreview] = useState<ArrayBuffer | string | null>(null);

  const handleClick = useCallback(() => {
    if (!inputRef.current) return;

    inputRef.current.click();
  }, []);

  const handleFileChange = useCallback(() => {
    if (!inputRef.current || !inputRef.current.files) return;

    const file = inputRef.current.files[0];

    const fr = new FileReader();
    fr.onload = () => {
      setImagePreview(fr.result);
    };

    fr.readAsDataURL(file);
  }, []);

  return (
    <div className="add-photo">
      {imagePreview ? (
        <div className="add-photo__confirm-wrapper">
          <div
            className="add-photo__preview"
            style={{ backgroundImage: `url(${imagePreview as string})` }}
          />
          <div className="add-photo__confirm-info">
            <p className="add-photo__confirm-text">Upload this picture?</p>
            <button className="add-photo__confirm-btn add-photo__confirm-btn--accept">
              Upload
            </button>
            <button className="add-photo__confirm-btn add-photo__confirm-btn--cancel">
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <button className="add-photo_button" onClick={handleClick}>
          <h1>Add photo</h1>
          <PlusIcon size="medium" />
        </button>
      )}
      <input
        ref={inputRef}
        type="file"
        accept="image/*"
        className="add-photo__input"
        onChange={handleFileChange}
      />
    </div>
  );
};

export default AddPhoto;

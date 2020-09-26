import React, { FC, useCallback, useRef, useState } from 'react';
import { connect } from 'react-redux';
import { PlusIcon } from '@primer/octicons-react';
import { addPhoto } from '../../store/albums';
import { getColorFrom } from '../../utils/chameleon';
import './styles.css';

const AddPhoto: FC<Props> = ({ addPhoto }) => {
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
      getColorFrom(fr.result as string).then((color) => console.log('popular', color));
      setImagePreview(fr.result);
    };

    fr.readAsDataURL(file);
  }, []);

  const handleConfirm = useCallback(() => {
    if (!inputRef.current || !inputRef.current.files) return;

    const file = inputRef.current.files[0];

    addPhoto(file);
  }, [addPhoto]);

  const handleCancel = useCallback(() => {
    if (!inputRef.current) return;

    inputRef.current.value = '';
    setImagePreview(null);
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
            <button
              className="add-photo__confirm-btn add-photo__confirm-btn--accept"
              onClick={handleConfirm}
            >
              Upload
            </button>
            <button
              className="add-photo__confirm-btn add-photo__confirm-btn--cancel"
              onClick={handleCancel}
            >
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

const mapDispatchToProps = {
  addPhoto: addPhoto.request,
};

type Props = typeof mapDispatchToProps;

export default connect(null, mapDispatchToProps)(AddPhoto);

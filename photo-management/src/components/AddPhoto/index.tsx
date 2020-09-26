import React, { FC, useCallback, useRef, useState } from 'react';
import { connect } from 'react-redux';
import { PlusIcon } from '@primer/octicons-react';
import { ApplicationState } from '../../store';
import { addPhoto, selectUploadStatus } from '../../store/albums';
import { getColorFrom } from '../../utils/chameleon';
import './styles.css';

const AddPhoto: FC<Props> = ({ addPhoto, status }) => {
  const inputRef = useRef<HTMLInputElement | null>(null);
  const [imagePreview, setImagePreview] = useState<{
    base64: ArrayBuffer | string;
    color: string;
  } | null>(null);

  const handleClick = useCallback(() => {
    if (!inputRef.current) return;

    inputRef.current.click();
  }, []);

  const handleFileChange = useCallback(() => {
    if (!inputRef.current || !inputRef.current.files) return;

    const file = inputRef.current.files[0];

    const fr = new FileReader();
    fr.onload = async () => {
      const color = await getColorFrom(fr.result as string);
      setImagePreview({ base64: fr.result as string, color });
    };

    fr.readAsDataURL(file);
  }, []);

  const handleConfirm = useCallback(() => {
    if (!inputRef.current || !inputRef.current.files || !imagePreview) return;

    const img = inputRef.current.files[0];

    addPhoto({ img, color: imagePreview.color });
  }, [addPhoto, imagePreview]);

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
            style={{ backgroundImage: `url(${imagePreview.base64})` }}
          />
          <div className="add-photo__confirm-info">
            <p className="add-photo__confirm-text">Upload this picture?</p>
            <button
              className="add-photo__confirm-btn add-photo__confirm-btn--accept"
              onClick={handleConfirm}
              disabled={status === 'loading'}
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

const mapStateToProps = (state: ApplicationState) => ({
  status: selectUploadStatus(state),
});

type Props = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(mapStateToProps, mapDispatchToProps)(AddPhoto);

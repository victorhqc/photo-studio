import React, { FC, FormEvent, useCallback, useEffect, useRef, useState } from 'react';
import { connect } from 'react-redux';
import { PlusIcon, SyncIcon } from '@primer/octicons-react';
import { ApplicationState } from '../../store';
import { addPhoto, selectUploadStatus } from '../../store/albums';
import { getColorFrom } from '../../utils/chameleon';
import PhotoGrid from '../PhotoGrid';
import './styles.css';

const AddPhoto: FC<Props> = ({ addPhoto, status }) => {
  const inputRef = useRef<HTMLInputElement | null>(null);
  const [imagePreview, setImagePreview] = useState<{
    base64: ArrayBuffer | string;
    color: string;
  } | null>(null);

  useEffect(() => {
    if (status === 'done' && inputRef.current) {
      inputRef.current.value = '';
      setImagePreview(null);
    }
  }, [status]);

  const handleClick = useCallback((e: FormEvent) => {
    e.preventDefault();

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

  const handleConfirm = useCallback(
    (e: FormEvent) => {
      e.preventDefault();

      if (!inputRef.current || !inputRef.current.files || !imagePreview) return;

      const img = inputRef.current.files[0];

      addPhoto({
        img,
        color: imagePreview.color,
      });
    },
    [addPhoto, imagePreview]
  );

  const handleCancel = useCallback((e: FormEvent) => {
    e.preventDefault();

    if (!inputRef.current) return;

    inputRef.current.value = '';
    setImagePreview(null);
  }, []);

  return (
    <form className="add-photo">
      <PhotoGrid>
        {status === 'loading' && (
          <div className="loading">
            <SyncIcon className="rotate-infinite" size="large" />
          </div>
        )}
        {imagePreview ? (
          <>
            <div
              className="add-photo__preview"
              style={{ backgroundImage: `url(${imagePreview.base64})` }}
            />
            <div className="add-photo__confirm-info">
              <div className="add-photo__btns-wrapper">
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
          </>
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
      </PhotoGrid>
    </form>
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

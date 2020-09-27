import React, { FC, FormEvent, useCallback, useEffect, useRef, useState } from 'react';
import { connect } from 'react-redux';
import { PlusIcon, SyncIcon } from '@primer/octicons-react';
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
  const [form, setForm] = useState<{ name: string; description: string }>({
    name: '',
    description: '',
  });

  useEffect(() => {
    if (status === 'done' && inputRef.current) {
      inputRef.current.value = '';
      setImagePreview(null);
      setForm({ name: '', description: '' });
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
        name: form.name,
        description: form.description || null,
      });
    },
    [addPhoto, imagePreview, form]
  );

  const handleCancel = useCallback((e: FormEvent) => {
    e.preventDefault();

    if (!inputRef.current) return;

    inputRef.current.value = '';
    setImagePreview(null);
    setForm({ name: '', description: '' });
  }, []);

  return (
    <form className="add-photo">
      {status === 'loading' && (
        <div className="loading rotate-infinite">
          <SyncIcon size="large" />
        </div>
      )}
      {imagePreview ? (
        <div className="add-photo__confirm-wrapper">
          <div
            className="add-photo__preview"
            style={{ backgroundImage: `url(${imagePreview.base64})` }}
          />
          <div className="add-photo__confirm-info">
            <h1 className="add-photo__confirm-title">Describe the photo</h1>
            <div className="input__wrapper">
              <label className="input__label" htmlFor="name">
                Name
              </label>
              <input
                className="input input--text"
                id="name"
                name="name"
                type="text"
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
              />
            </div>
            <div className="input__wrapper">
              <label className="input__label" htmlFor="description">
                Title
              </label>
              <textarea
                className="input input--textarea"
                rows={4}
                id="description"
                name="description"
                value={form.description}
                onChange={(e) => setForm({ ...form, description: e.target.value })}
              ></textarea>
            </div>
            <button
              className="add-photo__confirm-btn add-photo__confirm-btn--accept"
              onClick={handleConfirm}
              disabled={status === 'loading' || !form.name}
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

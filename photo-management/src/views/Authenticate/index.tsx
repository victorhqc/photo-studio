import { FC, useEffect } from 'react';
import { connect } from 'react-redux';
import { useLocation } from 'react-router';
import { parse } from 'qs';
import { authenticate } from '../../store/auth';

const Authorize: FC<Props> = ({ authenticate }) => {
  const location = useLocation();

  useEffect(() => {
    const search = parse(location.search.substring(1));

    if (search.token && typeof search.token === 'string') {
      authenticate(search.token);
    }
  }, [location.search, authenticate]);

  return null;
};

const mapDispatchToProps = {
  authenticate: authenticate.request,
};

type Props = typeof mapDispatchToProps;

export default connect(null, mapDispatchToProps)(Authorize);

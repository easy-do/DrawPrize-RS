import React, { useEffect, useState } from 'react';
import useLocale from '@/utils/useLocale';
import locale from './locale';
import styles from './style/index.module.less';
import LoginModal from '@/components/LoginModal';

export default function Welcome() {
  const t = useLocale(locale);
  // const userInfo = useSelector((state: any) => state.userInfo) || {};
  // const [openLoginModal, setOpenLoginModal] = useState(false);
  // useEffect(() => {
  //   if (!localStorage.getItem('Authorization')) {
  //     setOpenLoginModal(true);
  //   }
  // }, []);
  return (
    <div className={styles.container}>
      {t['welcome.title.welcome']}
      {/* <LoginModal visible={openLoginModal} setVisible={setOpenLoginModal} /> */}
    </div>
  );
}

Welcome.displayName = 'HomePage';
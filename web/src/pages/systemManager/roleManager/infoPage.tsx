import React, { useEffect, useState } from 'react';
import { Descriptions, Drawer, Spin } from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { getRoleInfo } from '@/api/role';
import dayjs from 'dayjs';

function InfoPage(props: { id: number; visible; setVisible }) {
  const [loading, setLoading] = useState(false);

  const [infoData, setInfoData] = useState<any>();

  function fetchData() {
    setLoading(true);
    if (props.id !== undefined) {
      getRoleInfo(props.id).then((res) => {
        const { success, data } = res.data;
        if (success) {
          setInfoData(data);
        }
      }).finally(()=>{
        setLoading(false);
      });
    }
  }

  useEffect(() => {
    fetchData();
  }, [props.id]);

  const t = useLocale(locale);

  return (
    <Drawer
      width={'30%'}
      title={t['searchTable.columns.view']}
      visible={props.visible}
      onOk={() => {
        props.setVisible(false);
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      footer={null}
    >
      <Spin dot loading={loading}>
        <Descriptions
          colon=""
          //   title='Personal Information'
          column={1}
          labelStyle={{ width: 100 }}
          data={[
            {
              label: t['searchTable.columns.id'],
              value: infoData ? infoData.id : '',
            },
            {
              label: t['searchTable.columns.role_name'],
              value: infoData ? infoData.role_name : '',
            },
            {
              label: t['searchTable.columns.role_code'],
              value: infoData ? infoData.role_code : '',
            },
          ]}
        />
      </Spin>
    </Drawer>
  );
}

export default InfoPage;

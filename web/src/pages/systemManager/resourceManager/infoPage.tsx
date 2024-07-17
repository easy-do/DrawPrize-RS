import React, { useEffect, useState } from 'react';
import { Descriptions, Drawer, Spin } from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { getResourceInfo } from '@/api/resource';

function InfoPage(props: { id: number; visible; setVisible }) {
  const [loading, setLoading] = useState(false);

  const [infoData, setInfoData] = useState<any>();

  function fetchData() {
    setLoading(true);
    if (props.id !== undefined) {
      getResourceInfo(props.id)
        .then((res) => {
          const { success, data } = res.data;
          if (success) {
            setInfoData(data);
          }
        })
        .finally(() => {
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
              label: t['searchTable.columns.parent_id'],
              value: infoData ? infoData.parent_id : '',
            },
            {
              label: t['searchTable.columns.resource_name'],
              value: infoData ? infoData.resource_name : '',
            },
            {
              label: t['searchTable.columns.resource_code'],
              value: infoData ? infoData.resource_code : '',
            },
            {
              label: t['searchTable.columns.resource_type'],
              value: infoData
                ? infoData.resource_type == 1
                  ? '菜单'
                  : '功能'
                : '',
            },
            {
              label: t['searchTable.columns.resource_root'],
              value: infoData
                ? infoData.resource_root
                  ? t['searchTable.columns.yes']
                  : t['searchTable.columns.no']
                : '',
            },
            {
              label: t['searchTable.columns.resource_action'],
              value: infoData
                ? infoData.resource_action
                  ? t['searchTable.columns.yes']
                  : t['searchTable.columns.no']
                : '',
            },
            {
              label: t['searchTable.columns.order_number'],
              value: infoData ? infoData.order_number : '',
            },
            {
              label: t['searchTable.columns.url'],
              value: infoData ? infoData.url : '',
            },
            {
              label: t['searchTable.columns.icon'],
              value: infoData ? infoData.icon : '',
            },
            {
              label: t['searchTable.columns.status'],
              value: infoData
              ? infoData.status
                ? t['searchForm.enable']
                : t['searchForm.disable']
              : '',
            },
            {
              label: t['searchTable.columns.api_path'],
              value: infoData ? infoData.api_path : '',
            },
            {
              label: t['searchTable.columns.api_http_method'],
              value: infoData ? infoData.api_http_method : '',
            },
            {
              label: t['searchTable.columns.api_path_regex'],
              value: infoData ? infoData.api_path_regex : '',
            },
            {
              label: t['searchTable.columns.role'],
              value: infoData ? infoData.role : '',
            },
            {
              label: t['searchTable.columns.resource_desc'],
              value: infoData ? infoData.resource_desc : '',
            },
          ]}
        />
      </Spin>
    </Drawer>
  );
}

export default InfoPage;

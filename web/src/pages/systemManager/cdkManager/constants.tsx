import React from 'react';
import { Button, Popconfirm, Typography } from '@arco-design/web-react';
import PermissionWrapper from '@/components/PermissionWrapper';
import dayjs from 'dayjs';

const { Text } = Typography;

export const DefaultSorter = {
  field: 'create_time',
  direction: 'desc',
};

export function getColumns(
  t: any,
  callback: (record: Record<string, any>, type: string) => Promise<void>
) {
  return [
    {
      title: t['searchTable.columns.id'],
      dataIndex: 'id',
      render: (value) => <Text copyable>{value}</Text>,
    },
    {
      title: t['searchTable.columns.code'],
      dataIndex: 'code',
      render: (value) => <Text copyable ={{text:value}}></Text>,
    },
    {
      title: t['searchTable.columns.use_status'],
      dataIndex: 'status',
      render: (value) =>
        value ? t['searchTable.columns.yes'] : t['searchTable.columns.no'],
    },
    {
      title: t['searchTable.columns.draw_prize_times'],
      dataIndex: 'ext_data',
      render: (value) =>
        value ? JSON.parse(value).draw_prize_times : '',
    },
    {
      title: t['searchTable.columns.create_time'],
      dataIndex: 'create_time',
      sorter: true,
      render: (x) => {
        if (x != undefined) {
          return dayjs(x).format('YYYY-MM-DD HH:mm:ss');
        }
        return x;
      },
    },
    {
      title: t['searchTable.columns.use_user'],
      dataIndex: 'use_user',
    },
    {
      title: t['searchTable.columns.use_time'],
      dataIndex: 'use_time',
      sorter: true,
      render: (x) => {
        if (x != undefined) {
          return dayjs(x).format('YYYY-MM-DD HH:mm:ss');
        }
        return x;
      },
    },
    {
      title: t['searchTable.columns.desc'],
      dataIndex: 'desc',
      ellipsis: true,
    },
    {
      title: t['searchTable.columns.operations'],
      dataIndex: 'operations',
      headerCellStyle: { paddingLeft: '15px' },
      render: (_, record) => [
        <PermissionWrapper
          key={'update'}
          requiredPermissions={[
            {
              resource: 'cdk_manager',
              actions: ['api_cdk_update'],
            },
          ]}
        >
          <Button
            disabled={record.use_status == 'true'}
            type="text"
            size="small"
            onClick={() => callback(record, 'update')}
          >
            {t['searchTable.columns.operations.update']}
          </Button>
        </PermissionWrapper>,
        <PermissionWrapper
        key={'delete'}
        requiredPermissions={[
          {
            resource: 'cdk_manager',
            actions: ['api_cdk_delete'],
          },
        ]}
      >
        <Popconfirm
          focusLock
          title={t['option.delete.confirm.title']}
          onOk={() => {
            callback(record, 'delete');
          }}
        >
          <Button 
          disabled={record.use_status == 'false'}
          status="danger" type="text" size="small">
            {t['searchTable.columns.operations.delete']}
          </Button>
        </Popconfirm>
      </PermissionWrapper>,,
      ],
    },
  ];
}

export default function Constants () {
  return (<></>)
}

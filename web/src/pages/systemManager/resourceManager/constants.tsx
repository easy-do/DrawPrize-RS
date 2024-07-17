import React from 'react';
import { Button, Typography, Popconfirm } from '@arco-design/web-react';
import PermissionWrapper from '@/components/PermissionWrapper';

const { Text } = Typography;

export const DefaultSorter = {
  field: 'id',
  direction: 'asc',
};

const RESOURCE_TYPE = ['', '菜单', '功能'];
export const API_HTTP_METHOD = [
  'GET',
  'POST',
  'PUT',
  'DELETE',
  'OPTIONS',
  'HEAD',
  'TRACE',
  'CONNECT',
  'PATCH',
];

export function getColumns(
  t: any,
  callback: (record: Record<string, any>, type: string) => Promise<void>
) {
  return [
    {
      title: t['searchTable.columns.id'],
      dataIndex: 'id',
      sorter: true,
      render: (value) => <Text copyable>{value}</Text>,
    },
    {
      title: t['searchTable.columns.resource_name'],
      dataIndex: 'resource_name',
    },
    {
      title: t['searchTable.columns.resource_code'],
      dataIndex: 'resource_code',
    },
    {
      title: t['searchTable.columns.resource_type'],
      dataIndex: 'resource_type',
      render: (value) => RESOURCE_TYPE[value],
    },
    {
      title: t['searchTable.columns.order_number'],
      dataIndex: 'order_number',
    },
    {
      title: t['searchTable.columns.status'],
      dataIndex: 'status',
      render: (value) =>
        value ? t['searchForm.enable'] : t['searchTable.disable'],
    },
    {
      title: t['searchTable.columns.resource_desc'],
      dataIndex: 'resource_desc',
      ellipsis: true,
      
    },
    {
      title: t['searchTable.columns.operations'],
      dataIndex: 'operations',
      headerCellStyle: { paddingLeft: '15px' },
      render: (_, record) => [
        <PermissionWrapper
          key={'view'}
          requiredPermissions={[
            { resource: 'resource_manager', actions: ['api_resource_info'] },
          ]}
        >
          <Button
            type="text"
            size="small"
            onClick={() => callback(record, 'view')}
          >
            {t['searchTable.columns.operations.view']}
          </Button>
        </PermissionWrapper>,
        <PermissionWrapper
          key={'update'}
          requiredPermissions={[
            { resource: 'resource_manager', actions: ['api_resource_update'] },
          ]}
        >
          <Button
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
            { resource: 'resource_manager', actions: ['api_resource_delete'] },
          ]}
        >
          <Popconfirm
            focusLock
            title={t['option.delete.confirm.title']}
            onOk={() => {
              callback(record, 'delete');
            }}
          >
            <Button status="danger" type="text" size="small">
              {t['searchTable.columns.operations.delete']}
            </Button>
          </Popconfirm>
        </PermissionWrapper>,
      ],
    },
  ];
}

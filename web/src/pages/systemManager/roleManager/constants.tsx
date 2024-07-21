import React from 'react';
import { Button, Typography, Popconfirm } from '@arco-design/web-react';
import PermissionWrapper from '@/components/PermissionWrapper';

const { Text } = Typography;

export const DefaultSorter = {
  field: 'id',
  direction: 'asc',
};

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
      title: t['searchTable.columns.role_name'],
      dataIndex: 'role_name',
    },
    {
      title: t['searchTable.columns.role_code'],
      dataIndex: 'role_code',
    },
    {
      title: t['searchTable.columns.operations'],
      dataIndex: 'operations',
      headerCellStyle: { paddingLeft: '15px' },
      render: (_, record) => [
        <PermissionWrapper
          key={'view'}
          requiredPermissions={[
            { resource: 'role_manager', actions: ['api_role_info'] },
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
            { resource: 'role_manager', actions: ['api_role_update'] },
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
          key={'set_resource'}
          requiredPermissions={[
            { resource: 'role_manager', actions: ['api_role_set_resource'] },
          ]}
        >
          <Button
            type="text"
            size="small"
            onClick={() => callback(record, 'set_role_resource')}
          >
            {t['searchTable.columns.operations.set_role_resource']}
          </Button>
        </PermissionWrapper>,
        <PermissionWrapper
          key={'delete'}
          requiredPermissions={[
            { resource: 'role_manager', actions: ['api_role_delete'] },
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
export default function Constants () {
  return (<></>)
}
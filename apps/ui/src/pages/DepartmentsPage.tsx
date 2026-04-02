import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { api } from "@/lib/api";
import type { PersistentAgent, Department } from "@/lib/types";

export default function DepartmentsPage() {
  const { id } = useParams();
  const [dept, setDept] = useState<Department | null>(null);
  const [members, setMembers] = useState<PersistentAgent[]>([]);
  const [allDepts, setAllDepts] = useState<Department[]>([]);

  useEffect(() => {
    api.getDepartments().then((d: any) => {
      const depts = d.departments || [];
      setAllDepts(depts);
      if (id) setDept(depts.find((dep: Department) => dep.id === id) || null);
    }).catch(() => {});

    api.getAgents().then((d: any) => {
      const agents = d.agents || [];
      if (id) setMembers(agents.filter((a: PersistentAgent) => a.department_id === id));
    }).catch(() => {});
  }, [id]);

  // Department detail view
  if (id && dept) {
    return (
      <div>
        <div className="dept-detail">
          <h2 className="dept-detail-name">{dept.name}</h2>
          <p className="dept-detail-meta">
            {members.length} agent{members.length !== 1 ? "s" : ""}
            {dept.manager_id && ` · Managed`}
          </p>

          {members.length > 0 && (
            <div className="dept-detail-members">
              {members.map((a) => (
                <div key={a.id} className="dept-detail-member">
                  <span className="dept-detail-member-name">
                    {a.display_name || a.name}
                  </span>
                  <span className="dept-detail-member-status">{a.status}</span>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    );
  }

  // Overview: all departments
  return (
    <div>
      <div className="dept-detail">
        <h2 className="dept-detail-name">Departments</h2>
        <p className="dept-detail-meta">{allDepts.length} department{allDepts.length !== 1 ? "s" : ""}</p>
      </div>
    </div>
  );
}
